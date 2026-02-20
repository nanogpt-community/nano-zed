use anyhow::{Result, anyhow};
use convert_case::{Case, Casing};
use futures::{AsyncReadExt, FutureExt, StreamExt, future::BoxFuture, stream};
use gpui::{AnyView, App, AsyncApp, Context, Entity, SharedString, Task, Window};
use http_client::{AsyncBody, HttpClient, Method, Request as HttpRequest};
use language_model::{
    ApiKeyState, AuthenticateError, EnvVar, IconOrSvg, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolSchemaFormat, RateLimiter,
};
use menu;
use open_ai::{
    ResponseStreamEvent,
    responses::{
        Request as ResponseRequest, StreamEvent as ResponsesStreamEvent,
        stream_response_with_headers,
    },
    stream_completion_with_headers,
};
use serde::Deserialize;
use settings::{Settings, SettingsStore};
use std::{collections::BTreeMap, sync::Arc};
use ui::{ElevationIndex, Tooltip, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::provider::open_ai::{
    OpenAiEventMapper, OpenAiResponseEventMapper, into_open_ai, into_open_ai_response,
};
pub use settings::OpenAiCompatibleAvailableModel as AvailableModel;
pub use settings::OpenAiCompatibleModelCapabilities as ModelCapabilities;

const NANOGPT_PROVIDER_ID: &str = "nanogpt";
const NANOGPT_API_KEY_ENV_VAR_NAME: &str = "NANOGPT_API_KEY";
const NANOGPT_DEFAULT_MODEL_ID: &str = "minimax/minimax-m2.5";
const NANOGPT_DEFAULT_MAX_INPUT_TOKENS: u64 = 200_000;

fn set_nanogpt_api_key_env_var(api_key: Option<&str>) {
    // SAFETY: This code intentionally mutates process environment variables to support the
    // NanoGPT-compatible client configuration path, and calls happen from serialized GPUI tasks.
    unsafe {
        match api_key {
            Some(api_key) => std::env::set_var(NANOGPT_API_KEY_ENV_VAR_NAME, api_key),
            None => std::env::remove_var(NANOGPT_API_KEY_ENV_VAR_NAME),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ResolvedModel {
    id: String,
    request_model: String,
    display_name: Option<String>,
    max_tokens: u64,
    max_output_tokens: Option<u64>,
    max_completion_tokens: Option<u64>,
    capabilities: ModelCapabilities,
    provider_override: Option<String>,
}

impl ResolvedModel {
    fn from_available_model(model: AvailableModel) -> Self {
        Self {
            id: model.name.clone(),
            request_model: model.name,
            display_name: model.display_name,
            max_tokens: model.max_tokens,
            max_output_tokens: model.max_output_tokens,
            max_completion_tokens: model.max_completion_tokens,
            capabilities: model.capabilities,
            provider_override: None,
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct OpenAiCompatibleSettings {
    pub api_url: String,
    pub available_models: Vec<AvailableModel>,
}

pub struct OpenAiCompatibleLanguageModelProvider {
    id: LanguageModelProviderId,
    name: LanguageModelProviderName,
    http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
}

pub struct State {
    id: Arc<str>,
    api_key_state: ApiKeyState,
    settings: OpenAiCompatibleSettings,
    http_client: Arc<dyn HttpClient>,
    dynamic_models: Vec<ResolvedModel>,
    fetch_dynamic_models_task: Option<Task<Result<(), LanguageModelCompletionError>>>,
    fetch_provider_selection_task: Option<Task<Result<(), LanguageModelCompletionError>>>,
}

impl State {
    fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    fn set_api_key(&mut self, api_key: Option<String>, cx: &mut Context<Self>) -> Task<Result<()>> {
        if self.is_nanogpt() {
            set_nanogpt_api_key_env_var(api_key.as_deref().filter(|value| !value.is_empty()));
        }

        let api_url = SharedString::new(self.settings.api_url.as_str());
        let store_task =
            self.api_key_state
                .store(api_url, api_key, |this| &mut this.api_key_state, cx);

        cx.spawn(async move |this, cx| {
            let result = store_task.await;
            this.update(cx, |this, cx| {
                this.sync_nanogpt_api_key_env();
                this.restart_dynamic_models_task(cx);
            })
            .ok();
            result
        })
    }

    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = SharedString::new(self.settings.api_url.clone());
        let authenticate_task =
            self.api_key_state
                .load_if_needed(api_url, |this| &mut this.api_key_state, cx);

        cx.spawn(async move |this, cx| {
            let result = authenticate_task.await;
            this.update(cx, |this, cx| {
                this.sync_nanogpt_api_key_env();
                this.restart_dynamic_models_task(cx);
            })
            .ok();
            result
        })
    }

    fn is_nanogpt(&self) -> bool {
        self.id.as_ref() == NANOGPT_PROVIDER_ID
    }

    fn sync_nanogpt_api_key_env(&self) {
        if !self.is_nanogpt() {
            return;
        }

        set_nanogpt_api_key_env_var(self.api_key_state.key(&self.settings.api_url).as_deref());
    }

    fn restart_dynamic_models_task(&mut self, cx: &mut Context<Self>) {
        if !self.is_nanogpt() {
            self.dynamic_models.clear();
            self.fetch_dynamic_models_task = None;
            self.fetch_provider_selection_task = None;
            return;
        }

        if !self.is_authenticated() {
            self.dynamic_models.clear();
            self.fetch_dynamic_models_task = None;
            self.fetch_provider_selection_task = None;
            cx.notify();
            return;
        }

        let task = self.fetch_dynamic_models(cx);
        self.fetch_dynamic_models_task = Some(task);
    }

    fn fetch_dynamic_models(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), LanguageModelCompletionError>> {
        let http_client = self.http_client.clone();
        let api_url = self.settings.api_url.clone();
        let api_key = self.api_key_state.key(&api_url);

        cx.spawn(async move |this, cx| {
            let models = fetch_nanogpt_models(http_client.as_ref(), &api_url, api_key.as_deref())
                .await
                .map_err(LanguageModelCompletionError::Other)?;

            this.update(cx, |this, cx| {
                this.dynamic_models = models.clone();
                cx.notify();
                this.restart_provider_selection_task(models, cx);
            })
            .map_err(LanguageModelCompletionError::Other)?;

            Ok(())
        })
    }

    fn restart_provider_selection_task(
        &mut self,
        models: Vec<ResolvedModel>,
        cx: &mut Context<Self>,
    ) {
        if models.is_empty() {
            self.fetch_provider_selection_task = None;
            return;
        }

        let task = self.fetch_provider_selection_models(models, cx);
        self.fetch_provider_selection_task = Some(task);
    }

    fn fetch_provider_selection_models(
        &mut self,
        models: Vec<ResolvedModel>,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), LanguageModelCompletionError>> {
        let http_client = self.http_client.clone();
        let api_url = self.settings.api_url.clone();
        let api_key = self.api_key_state.key(&api_url);

        cx.spawn(async move |this, cx| {
            let provider_options = stream::iter(models.iter().cloned())
                .map(|model| {
                    let http_client = http_client.clone();
                    let api_url = api_url.clone();
                    let api_key = api_key.clone();
                    async move {
                        let providers = match fetch_nanogpt_model_providers(
                            http_client.as_ref(),
                            &api_url,
                            &model.request_model,
                            api_key.as_deref(),
                        )
                        .await
                        {
                            Ok(providers) => providers,
                            Err(error) => {
                                log::warn!(
                                    "Failed fetching NanoGPT provider options for model {}: {error:#}",
                                    model.request_model
                                );
                                Vec::new()
                            }
                        };
                        (model, providers)
                    }
                })
                .buffer_unordered(8)
                .collect::<Vec<_>>()
                .await;

            let mut models_with_provider_options = Vec::new();
            for (model, providers) in provider_options {
                models_with_provider_options.push(model.clone());
                for provider in providers {
                    if provider.is_empty() {
                        continue;
                    }

                    models_with_provider_options.push(ResolvedModel {
                        id: format!("{}@{}", model.request_model, provider),
                        request_model: model.request_model.clone(),
                        display_name: Some(format!(
                            "{} via {}",
                            model
                                .display_name
                                .as_deref()
                                .unwrap_or(model.request_model.as_str()),
                            provider
                        )),
                        max_tokens: model.max_tokens,
                        max_output_tokens: model.max_output_tokens,
                        max_completion_tokens: model.max_completion_tokens,
                        capabilities: model.capabilities.clone(),
                        provider_override: Some(provider),
                    });
                }
            }

            this.update(cx, |this, cx| {
                this.dynamic_models = models_with_provider_options;
                cx.notify();
            })
            .map_err(LanguageModelCompletionError::Other)?;

            Ok(())
        })
    }

    fn resolved_models(&self) -> Vec<ResolvedModel> {
        let mut models = if self.is_nanogpt() && !self.dynamic_models.is_empty() {
            self.dynamic_models.clone()
        } else {
            Vec::new()
        };

        for model in self.settings.available_models.iter().cloned() {
            let resolved_model = ResolvedModel::from_available_model(model);
            if !models
                .iter()
                .any(|existing| existing.id == resolved_model.id)
            {
                models.push(resolved_model);
            }
        }

        models
    }
}

impl OpenAiCompatibleLanguageModelProvider {
    pub fn new(id: Arc<str>, http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        fn resolve_settings<'a>(id: &'a str, cx: &'a App) -> Option<&'a OpenAiCompatibleSettings> {
            crate::AllLanguageModelSettings::get_global(cx)
                .openai_compatible
                .get(id)
        }

        let api_key_env_var_name = format!("{}_API_KEY", id).to_case(Case::UpperSnake).into();
        let state = cx.new(|cx| {
            cx.observe_global::<SettingsStore>(|this: &mut State, cx| {
                let Some(settings) = resolve_settings(&this.id, cx).cloned() else {
                    return;
                };
                if &this.settings != &settings {
                    let api_url = SharedString::new(settings.api_url.as_str());
                    this.api_key_state.handle_url_change(
                        api_url,
                        |this| &mut this.api_key_state,
                        cx,
                    );
                    this.settings = settings;
                    this.sync_nanogpt_api_key_env();
                    this.restart_dynamic_models_task(cx);
                    cx.notify();
                }
            })
            .detach();
            let settings = resolve_settings(&id, cx).cloned().unwrap_or_default();
            State {
                id: id.clone(),
                api_key_state: ApiKeyState::new(
                    SharedString::new(settings.api_url.as_str()),
                    EnvVar::new(api_key_env_var_name),
                ),
                settings,
                http_client: http_client.clone(),
                dynamic_models: Vec::new(),
                fetch_dynamic_models_task: None,
                fetch_provider_selection_task: None,
            }
        });

        if id.as_ref() == NANOGPT_PROVIDER_ID {
            state
                .update(cx, |state, cx| state.authenticate(cx))
                .detach();
        }

        Self {
            id: id.clone().into(),
            name: id.into(),
            http_client,
            state,
        }
    }

    fn create_language_model(&self, model: ResolvedModel) -> Arc<dyn LanguageModel> {
        Arc::new(OpenAiCompatibleLanguageModel {
            id: LanguageModelId::from(model.id.clone()),
            provider_id: self.id.clone(),
            provider_name: self.name.clone(),
            model,
            state: self.state.clone(),
            http_client: self.http_client.clone(),
            request_limiter: RateLimiter::new(4),
        })
    }
}

impl LanguageModelProviderState for OpenAiCompatibleLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl LanguageModelProvider for OpenAiCompatibleLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelProviderName {
        self.name.clone()
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiOpenAiCompat)
    }

    fn default_model(&self, cx: &App) -> Option<Arc<dyn LanguageModel>> {
        let state = self.state.read(cx);
        let models = state.resolved_models();
        let default_model = if state.is_nanogpt() {
            models
                .iter()
                .find(|model| {
                    model.request_model == NANOGPT_DEFAULT_MODEL_ID
                        && model.provider_override.is_none()
                })
                .cloned()
                .or_else(|| models.first().cloned())
        } else {
            models.first().cloned()
        };
        default_model.map(|model| self.create_language_model(model))
    }

    fn default_fast_model(&self, _cx: &App) -> Option<Arc<dyn LanguageModel>> {
        None
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        self.state
            .read(cx)
            .resolved_models()
            .iter()
            .map(|model| self.create_language_model(model.clone()))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(
        &self,
        _target_agent: language_model::ConfigurationViewTargetAgent,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.set_api_key(None, cx))
    }
}

pub struct OpenAiCompatibleLanguageModel {
    id: LanguageModelId,
    provider_id: LanguageModelProviderId,
    provider_name: LanguageModelProviderName,
    model: ResolvedModel,
    state: Entity<State>,
    http_client: Arc<dyn HttpClient>,
    request_limiter: RateLimiter,
}

impl OpenAiCompatibleLanguageModel {
    fn stream_completion(
        &self,
        mut request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<'static, Result<ResponseStreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
            )
        });

        if self.model.provider_override.is_some() {
            request.billing_mode = Some("paygo".to_string());
        }

        let additional_headers =
            self.model
                .provider_override
                .as_ref()
                .map_or_else(Vec::new, |provider| {
                    vec![
                        ("X-Provider".to_string(), provider.to_string()),
                        ("X-Billing-Mode".to_string(), "paygo".to_string()),
                    ]
                });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_completion_with_headers(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &additional_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_response(
        &self,
        mut request: ResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<futures::stream::BoxStream<'static, Result<ResponsesStreamEvent>>>>
    {
        let http_client = self.http_client.clone();

        let (api_key, api_url) = self.state.read_with(cx, |state, _cx| {
            let api_url = &state.settings.api_url;
            (
                state.api_key_state.key(api_url),
                state.settings.api_url.clone(),
            )
        });

        if self.model.provider_override.is_some() {
            request.billing_mode = Some("paygo".to_string());
        }

        let additional_headers =
            self.model
                .provider_override
                .as_ref()
                .map_or_else(Vec::new, |provider| {
                    vec![
                        ("X-Provider".to_string(), provider.to_string()),
                        ("X-Billing-Mode".to_string(), "paygo".to_string()),
                    ]
                });

        let provider = self.provider_name.clone();
        let future = self.request_limiter.stream(async move {
            let Some(api_key) = api_key else {
                return Err(LanguageModelCompletionError::NoApiKey { provider });
            };
            let request = stream_response_with_headers(
                http_client.as_ref(),
                provider.0.as_str(),
                &api_url,
                &api_key,
                request,
                &additional_headers,
            );
            let response = request.await?;
            Ok(response)
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

impl LanguageModel for OpenAiCompatibleLanguageModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(
            self.model
                .display_name
                .clone()
                .unwrap_or_else(|| self.model.request_model.clone()),
        )
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        self.provider_id.clone()
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        self.provider_name.clone()
    }

    fn supports_tools(&self) -> bool {
        self.model.capabilities.tools
    }

    fn tool_input_format(&self) -> LanguageModelToolSchemaFormat {
        LanguageModelToolSchemaFormat::JsonSchemaSubset
    }

    fn supports_images(&self) -> bool {
        self.model.capabilities.images
    }

    fn supports_tool_choice(&self, choice: LanguageModelToolChoice) -> bool {
        match choice {
            LanguageModelToolChoice::Auto => self.model.capabilities.tools,
            LanguageModelToolChoice::Any => self.model.capabilities.tools,
            LanguageModelToolChoice::None => true,
        }
    }

    fn supports_split_token_display(&self) -> bool {
        true
    }

    fn telemetry_id(&self) -> String {
        if let Some(provider_override) = self.model.provider_override.as_deref() {
            format!("openai/{}@{}", self.model.request_model, provider_override)
        } else {
            format!("openai/{}", self.model.request_model)
        }
    }

    fn max_token_count(&self) -> u64 {
        self.model.max_tokens
    }

    fn max_output_tokens(&self) -> Option<u64> {
        self.model.max_output_tokens
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<u64>> {
        let max_token_count = self.max_token_count();
        cx.background_spawn(async move {
            let messages = super::open_ai::collect_tiktoken_messages(request);
            let model = if max_token_count >= 100_000 {
                // If the max tokens is 100k or more, it is likely the o200k_base tokenizer from gpt4o
                "gpt-4o"
            } else {
                // Otherwise fallback to gpt-4, since only cl100k_base and o200k_base are
                // supported with this tiktoken method
                "gpt-4"
            };
            tiktoken_rs::num_tokens_from_messages(model, &messages).map(|tokens| tokens as u64)
        })
        .boxed()
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            futures::stream::BoxStream<
                'static,
                Result<LanguageModelCompletionEvent, LanguageModelCompletionError>,
            >,
            LanguageModelCompletionError,
        >,
    > {
        if self.model.capabilities.chat_completions {
            let request = into_open_ai(
                request,
                &self.model.request_model,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                None,
            );
            let completions = self.stream_completion(request, cx);
            async move {
                let mapper = OpenAiEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        } else {
            let request = into_open_ai_response(
                request,
                &self.model.request_model,
                self.model.capabilities.parallel_tool_calls,
                self.model.capabilities.prompt_cache_key,
                self.max_output_tokens(),
                None,
            );
            let completions = self.stream_response(request, cx);
            async move {
                let mapper = OpenAiResponseEventMapper::new();
                Ok(mapper.map_stream(completions.await?).boxed())
            }
            .boxed()
        }
    }
}

#[derive(Default, Deserialize)]
struct NanogptModelsResponse {
    #[serde(default)]
    models: NanogptModelCollections,
}

#[derive(Default, Deserialize)]
struct NanogptModelCollections {
    #[serde(default)]
    text: BTreeMap<String, NanogptCatalogModel>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NanogptCatalogModel {
    model: Option<String>,
    name: Option<String>,
    visible: Option<bool>,
    max_input_tokens: Option<u64>,
    max_output_tokens: Option<u64>,
    #[serde(default)]
    capabilities: Vec<String>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NanogptProvidersResponse {
    #[serde(default)]
    supports_provider_selection: bool,
    #[serde(default)]
    providers: Vec<NanogptProviderInfo>,
}

#[derive(Default, Deserialize)]
struct NanogptProviderInfo {
    provider: String,
    #[serde(default)]
    available: bool,
}

fn nanogpt_capabilities(capabilities: &[String]) -> ModelCapabilities {
    let has_capability = |capability: &str| {
        capabilities
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(capability))
    };
    let tools = has_capability("tool-calling");
    ModelCapabilities {
        tools,
        images: has_capability("vision"),
        parallel_tool_calls: tools,
        prompt_cache_key: false,
        chat_completions: true,
    }
}

fn nanogpt_api_base_url(api_url: &str) -> String {
    let trimmed = api_url.trim_end_matches('/');
    if let Some(stripped) = trimmed.strip_suffix("/v1") {
        stripped.to_string()
    } else {
        trimmed.to_string()
    }
}

async fn fetch_nanogpt_models(
    http_client: &dyn HttpClient,
    api_url: &str,
    api_key: Option<&str>,
) -> Result<Vec<ResolvedModel>> {
    let uri = format!("{}/models?detailed=true", nanogpt_api_base_url(api_url));
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");
    if let Some(api_key) = api_key
        && !api_key.is_empty()
    {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
    }
    let request = request_builder
        .body(AsyncBody::default())
        .map_err(|error| anyhow!(error))?;

    let mut response = http_client.send(request).await?;
    let status_code = response.status();
    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(|error| anyhow!(error))?;

    if !status_code.is_success() {
        return Err(anyhow!(
            "NanoGPT model listing request failed with status {}: {}",
            status_code,
            body
        ));
    }

    let mut models = Vec::new();
    let response: NanogptModelsResponse = serde_json::from_str(&body)?;
    for (key, model) in response.models.text {
        if model.visible == Some(false) {
            continue;
        }

        let request_model = model.model.unwrap_or(key);
        let max_tokens = model
            .max_input_tokens
            .filter(|max_tokens| *max_tokens > 0)
            .unwrap_or(NANOGPT_DEFAULT_MAX_INPUT_TOKENS);
        let max_output_tokens = model.max_output_tokens.filter(|max_tokens| *max_tokens > 0);

        models.push(ResolvedModel {
            id: request_model.clone(),
            request_model,
            display_name: model.name,
            max_tokens,
            max_output_tokens,
            max_completion_tokens: max_output_tokens,
            capabilities: nanogpt_capabilities(&model.capabilities),
            provider_override: None,
        });
    }

    models.sort_by(|left, right| {
        if left.request_model == NANOGPT_DEFAULT_MODEL_ID {
            return std::cmp::Ordering::Less;
        }
        if right.request_model == NANOGPT_DEFAULT_MODEL_ID {
            return std::cmp::Ordering::Greater;
        }
        left.display_name
            .as_deref()
            .unwrap_or(left.request_model.as_str())
            .cmp(
                right
                    .display_name
                    .as_deref()
                    .unwrap_or(right.request_model.as_str()),
            )
    });

    Ok(models)
}

async fn fetch_nanogpt_model_providers(
    http_client: &dyn HttpClient,
    api_url: &str,
    model_id: &str,
    api_key: Option<&str>,
) -> Result<Vec<String>> {
    let uri = format!(
        "{}/models/{}/providers",
        nanogpt_api_base_url(api_url),
        urlencoding::encode(model_id)
    );
    let mut request_builder = HttpRequest::builder()
        .method(Method::GET)
        .uri(uri)
        .header("Accept", "application/json");
    if let Some(api_key) = api_key
        && !api_key.is_empty()
    {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
    }
    let request = request_builder
        .body(AsyncBody::default())
        .map_err(|error| anyhow!(error))?;

    let mut response = http_client.send(request).await?;
    let status_code = response.status();
    let mut body = String::new();
    response
        .body_mut()
        .read_to_string(&mut body)
        .await
        .map_err(|error| anyhow!(error))?;

    if !status_code.is_success() {
        return Err(anyhow!(
            "NanoGPT provider listing request failed with status {} for model {}: {}",
            status_code,
            model_id,
            body
        ));
    }

    let response: NanogptProvidersResponse = serde_json::from_str(&body)?;
    if !response.supports_provider_selection {
        return Ok(Vec::new());
    }

    let mut providers = response
        .providers
        .into_iter()
        .filter_map(|provider_info| {
            if provider_info.available && !provider_info.provider.is_empty() {
                Some(provider_info.provider)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    providers.sort();
    providers.dedup();
    Ok(providers)
}

struct ConfigurationView {
    api_key_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let api_key_editor = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "000000000000000000000000000000000000000000000000000",
            )
        });

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
                    if let Err(error) = task.await {
                        log::debug!(
                            "OpenAI-compatible provider authentication failed while loading credentials: {error:#}"
                        );
                    }
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor,
            state,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let env_var_name = state.api_key_state.env_var_name();
        let setup_message = if state.is_nanogpt() {
            "To use nano-zed's agent with NanoGPT, you need to add a NanoGPT API key."
        } else {
            "To use nano-zed's agent with an OpenAI-compatible provider, you need to add an API key."
        };

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(setup_message))
                .child(
                    div()
                        .pt(DynamicSpacing::Base04.rems(cx))
                        .child(self.api_key_editor.clone())
                )
                .child(
                    Label::new(
                        format!(
                            "You can also set the {env_var_name} environment variable and restart nano-zed."
                        ),
                    )
                    .size(LabelSize::Small).color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .flex_1()
                        .min_w_0()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(
                            div()
                                .w_full()
                                .overflow_x_hidden()
                                .text_ellipsis()
                                .child(Label::new(
                                    if env_var_set {
                                        format!("API key set in {env_var_name} environment variable")
                                    } else {
                                        format!("API key configured for {}", &state.settings.api_url)
                                    }
                                ))
                        ),
                )
                .child(
                    h_flex()
                        .flex_shrink_0()
                        .child(
                            Button::new("reset-api-key", "Reset API Key")
                                .label_size(LabelSize::Small)
                                .icon(IconName::Undo)
                                .icon_size(IconSize::Small)
                                .icon_position(IconPosition::Start)
                                .layer(ElevationIndex::ModalSurface)
                                .when(env_var_set, |this| {
                                    this.tooltip(Tooltip::text(format!("To reset your API key, unset the {env_var_name} environment variable.")))
                                })
                                .on_click(cx.listener(|this, _, window, cx| this.reset_api_key(window, cx))),
                        ),
                )
                .into_any()
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            v_flex().size_full().child(api_key_section).into_any()
        }
    }
}
