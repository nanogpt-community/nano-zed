### Start Gemini CLI (Bash)

Source: https://docs.nano-gpt.com/integrations/gemini-cli

Launches the Gemini CLI from the `gemini-cli` directory after installation and configuration. May prompt for theme selection or authentication on first run.

```bash
npm start
```

--------------------------------

### NanoGPT MCP Server Installation

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/mcp-server

Instructions on how to install and run the NanoGPT MCP Server using npx, and how to add it to Claude Code and IDEs.

```APIDOC
## NanoGPT MCP Server Installation

### Description
Install and run the NanoGPT MCP server directly using `npx`. No manual installation is required if you have Node.js installed.

### Method
Command Line

### Endpoint
N/A

### Parameters
None

### Request Example
```bash
npx @nanogpt/mcp
```

### Claude Code Integration

#### Description
Add NanoGPT MCP to Claude Code via the CLI.

#### Method
Command Line

#### Endpoint
N/A

#### Parameters
* `nanogpt`: The name for the MCP server.
* `npx`: The command to execute.
* `@nanogpt/mcp`: The package to run.
* `--scope user`: Specifies the scope for the addition.
* `--env NANOGPT_API_KEY=YOUR_API_KEY`: Sets the required API key environment variable.

#### Request Example
```bash
claude mcp add nanogpt "npx" "@nanogpt/mcp" --scope user \
  --env NANOGPT_API_KEY=YOUR_API_KEY
```

### IDE Integration

#### Description
Configure NanoGPT MCP for IDEs like Claude Desktop, Cline, or Cursor by adding it to your configuration file.

#### Method
Configuration File (JSON)

#### Endpoint
N/A

#### Parameters
* `mcpServers.nanogpt.command`: The command to execute (`npx`).
* `mcpServers.nanogpt.args`: Arguments for the command (`["-y", "@nanogpt/mcp"]`).
* `mcpServers.nanogpt.env.NANOGPT_API_KEY`: Your NanoGPT API key.

#### Request Example
```json
{
  "mcpServers": {
    "nanogpt": {
      "command": "npx",
      "args": ["-y", "@nanogpt/mcp"],
      "env": {
        "NANOGPT_API_KEY": "your_api_key_here"
      }
    }
  }
}
```
```

--------------------------------

### API Authentication Examples (Bash)

Source: https://docs.nano-gpt.com/api-reference/video-generation

Demonstrates how to authenticate API requests using either the 'x-api-key' header or the 'Authorization: Bearer' token. These examples are useful for interacting with the API from the command line.

```bash
# Using x-api-key header
curl -H "x-api-key: YOUR_API_KEY"

# Using Bearer token
curl -H "Authorization: Bearer YOUR_API_KEY"
```

--------------------------------

### Start Codex CLI

Source: https://docs.nano-gpt.com/integrations/codex-cli

Starts the Codex CLI after it has been installed and configured. This command will launch the interactive CLI session.

```bash
codex
```

--------------------------------

### Nano-GPT API Chat Completions with Exa Web Search

Source: https://docs.nano-gpt.com/api-reference/endpoint/chat-completion

Demonstrates how to perform chat completions using the Nano-GPT API, with specific examples for configuring the Exa web search provider. This includes setting parameters like 'depth' and 'numResults' for more targeted searches. The examples show how to structure the request body for different search scenarios.

```python
import requests
import json

BASE_URL = "https://nano-gpt.com/api/v1"
API_KEY = "YOUR_API_KEY"

headers = {
    "Authorization": f"Bearer {API_KEY}",
    "Content-Type": "application/json"
}

# Suffix-based standard web search
data = {
    "model": "openai/gpt-5.2:online",
    "messages": [
        {"role": "user", "content": "What are the latest developments in AI?"}
    ]
}

response = requests.post(
    f"{BASE_URL}/chat/completions",
    headers=headers,
    json=data
)

# Request-body configuration (Exa neural)
data_search = {
    "model": "openai/gpt-5.2",
    "messages": [
        {"role": "user", "content": "Provide a comprehensive analysis of recent AI breakthroughs"}
    ],
    "webSearch": {
        "enabled": True,
        "provider": "exa",
        "depth": "neural",
        "numResults": 10
    }
}
```

```javascript
const BASE_URL = "https://nano-gpt.com/api/v1";
const API_KEY = "YOUR_API_KEY";

// Suffix-based standard web search
const response = await fetch(`${BASE_URL}/chat/completions`, {
    method: 'POST',
    headers: {
        'Authorization': `Bearer ${API_KEY}`,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        model: 'openai/gpt-5.2:online',
        messages: [
            { role: 'user', content: 'What are the latest developments in AI?' }
        ]
    })
});

// Request-body configuration (Exa neural)
const searchResponse = await fetch(`${BASE_URL}/chat/completions`, {
    method: 'POST',
    headers: {
        'Authorization': `Bearer ${API_KEY}`,
        'Content-Type': 'application/json'
    },
    body: JSON.stringify({
        model: 'openai/gpt-5.2',
        messages: [
            { role: 'user', content: 'Provide a comprehensive analysis of recent AI breakthroughs' }
        ],
        webSearch: {
            enabled: true,
            provider: 'exa',
            depth: 'neural',
            numResults: 10
        }
    })
});
```

```bash
# Suffix-based standard web search
curl -X POST https://nano-gpt.com/api/v1/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "openai/gpt-5.2:online",
    "messages": [
      {"role": "user", "content": "What are the latest developments in AI?"}
    ]
  }'

# Request-body configuration (Exa neural)
curl -X POST https://nano-gpt.com/api/v1/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "openai/gpt-5.2",
    "messages": [
      {"role": "user", "content": "Provide a comprehensive analysis of recent AI breakthroughs"}
    ],
    "webSearch": {
      "enabled": true,
      "provider": "exa",
      "depth": "neural",
      "numResults": 10
    }
}'
```

--------------------------------

### One-Click OpenCode Setup for NanoGPT (Mac/Linux)

Source: https://docs.nano-gpt.com/integrations/opencode

This command-line script automates the setup of OpenCode with NanoGPT. It handles browser authentication or API key pasting, configures OpenCode to use NanoGPT as its AI provider, and sets up popular models.

```bash
curl -fsSL "https://nano-gpt.com/install/opencode_nanogpt.sh" | bash
```

--------------------------------

### Install Grok CLI

Source: https://docs.nano-gpt.com/integrations/grok-cli

Installs the Grok CLI globally using npm and verifies the installation by checking the version. Ensure Node.js 18 or newer is installed. If permission issues arise, use 'sudo' or run as administrator.

```bash
# Install Grok CLI
npm install -g @vibe-kit/grok-cli

# Verify installation
grok --version
```

--------------------------------

### Configure nano-gpt Models in ClawdBot (JSON)

Source: https://docs.nano-gpt.com/integrations/clawdbot

This JSON configuration enables various nano-gpt models within ClawdBot. It requires a valid API key from nano-gpt.com and specifies model details such as baseUrl, apiKey, api type, and model-specific parameters like contextWindow and maxTokens. Ensure the 'YOUR_NANOGPT_API_KEY_HERE' placeholder is replaced with your actual API key.

```json
{
  "models": {
    "providers": {
      "nanogpt": {
        "baseUrl": "https://nano-gpt.com/api/v1",
        "apiKey": "YOUR_NANOGPT_API_KEY_HERE",
        "auth": "api-key",
        "api": "openai-completions",
        "headers": {},
        "authHeader": false,
        "models": [
          {
            "id": "anthropic/claude-opus-4.5",
            "name": "Claude Opus 4.5",
            "api": "openai-completions",
            "reasoning": false,
            "contextWindow": 200000,
            "input": ["text"],
            "cost": {
              "input": 0.015,
              "output": 0.075,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "openai/gpt-5.2",
            "name": "GPT 5.2",
            "api": "openai-completions",
            "reasoning": false,
            "contextWindow": 128000,
            "input": ["text"],
            "cost": {
              "input": 0.003,
              "output": 0.012,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "google/gemini-3-flash-preview",
            "name": "Gemini 3 Flash Preview",
            "api": "openai-completions",
            "reasoning": false,
            "contextWindow": 1000000,
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "minimax/minimax-m2.1",
            "name": "MiniMax M2.1",
            "api": "openai-completions",
            "reasoning": false,
            "contextWindow": 1000000,
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "moonshotai/kimi-k2-thinking",
            "name": "Kimi K2 Thinking",
            "api": "openai-completions",
            "reasoning": true,
            "contextWindow": 128000,
            "compat": {
              "supportsReasoningEffort": true
            },
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "meta-llama/llama-3.3-70b-instruct",
            "name": "Llama 3.3 70B Instruct",
            "api": "openai-completions",
            "reasoning": false,
            "contextWindow": 128000,
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "qwen/qwen3-235b-thinking",
            "name": "Qwen3 235B Thinking",
            "api": "openai-completions",
            "reasoning": true,
            "contextWindow": 32000,
            "compat": {
              "supportsReasoningEffort": true
            },
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "qwen/qwen3-30b",
            "name": "Qwen3 30B",
            "api": "openai-completions",
            "reasoning": false,
            "contextWindow": 32000,
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "zai-org/glm-4.7:thinking",
            "name": "GLM 4.7 (Thinking)",
            "api": "openai-completions",
            "reasoning": true,
            "contextWindow": 200000,
            "compat": {
              "supportsReasoningEffort": true
            },
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          },
          {
            "id": "zai-org/glm-4.7",
            "name": "GLM 4.7 (Standard)",
            "api": "openai-completions",
            "reasoning": false,
            "contextWindow": 200000,
            "input": ["text"],
            "cost": {
              "input": 0,
              "output": 0,
              "cacheRead": 0,
              "cacheWrite": 0
            },
            "maxTokens": 8192
          }
        ]
      }
    }
  }
}
```

--------------------------------

### Music Generation Examples

Source: https://docs.nano-gpt.com/api-reference/music-generation

Examples of how to generate music using NanoGPT's API, provided in cURL and Python (using the OpenAI SDK).

```bash
curl -X POST https://nano-gpt.com/api/v1/audio/speech \
  -H "Authorization: Bearer $NANOGPT_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "YOUR_MUSIC_MODEL_ID",
    "input": "An energetic electronic dance track with heavy bass drops and synth leads"
  }' \
  --output music.mp3
```

```python
from openai import OpenAI

client = OpenAI(
    base_url="https://nano-gpt.com/api/v1",
    api_key="YOUR_API_KEY",
)

response = client.audio.speech.create(
    model="YOUR_MUSIC_MODEL_ID",
    input="A peaceful acoustic guitar melody with soft drums",
    voice="alloy",  # Ignored for music models; included for OpenAI SDK compatibility
)

response.stream_to_file("output.mp3")
```

--------------------------------

### Install NanoGPT MCP Server using npx

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/mcp-server

This command installs and runs the NanoGPT MCP server directly using Node Package Execute (npx). It requires Node.js to be installed on the system. No manual installation steps are needed.

```bash
npx @nanogpt/mcp
```

--------------------------------

### Replace API Key in ClawdBot Configuration

Source: https://docs.nano-gpt.com/integrations/clawdbot

This snippet shows how to replace a placeholder API key with your actual NanoGPT API key within the ClawdBot configuration. Ensure you replace 'YOUR_NANOGPT_API_KEY_HERE' with your valid key.

```json
{
  "input": ["text"],
  "cost": {
    "input": 0,
    "output": 0,
    "cacheRead": 0,
    "cacheWrite": 0
  },
  "maxTokens": 8192
}
```

5. Replace `YOUR_NANOGPT_API_KEY_HERE` with your actual NanoGPT API key
6. Save and restart ClawdBot
```

--------------------------------

### API Request Example with Logit Shaping and Determinism Parameters

Source: https://docs.nano-gpt.com/api-reference/endpoint/chat-completion

This example demonstrates a cURL request to the NanoGPT API's chat completions endpoint. It includes various parameters for controlling generation, such as temperature, top_p, top_k, and seed for determinism. The seed parameter ensures repeatable completions when supported by the provider.

```bash
curl -X POST https://nano-gpt.com/api/v1/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "google/gemini-3-flash-preview",
    "messages": [{"role": "user", "content": "Write a creative story about space exploration"}],
    "temperature": 0.8,
    "top_p": 0.9,
    "top_k": 40,
    "tfs": 0.8,
    "typical_p": 0.95,
    "mirostat_mode": 2,
    "mirostat_tau": 5,
    "mirostat_eta": 0.1,
    "max_tokens": 500,
    "frequency_penalty": 0.3,
    "presence_penalty": 0.1,
    "repetition_penalty": 1.1,
    "stop": ["###"],
    "seed": 42
  }'
```

--------------------------------

### Enable Web Search using Python

Source: https://docs.nano-gpt.com/quickstart

This Python example demonstrates how to enable real-time web search for a model by appending a suffix to the model name. It shows a standard web search request and prepares for a deep web search request.

```python
import requests
import json

BASE_URL = "https://nano-gpt.com/api/v1"
API_KEY = "YOUR_API_KEY"

headers = {
    "Authorization": f"Bearer {API_KEY}",
    "Content-Type": "application/json"
}

# Standard web search ($0.006 per request)
data = {
    "model": "openai/gpt-5.2:online",
    "messages": [
        {"role": "user", "content": "What are the latest AI announcements this week?"}
    ],
    "stream": False
}

response = requests.post(
    f"{BASE_URL}/chat/completions",
    headers=headers,
    json=data
)

print("Response:", response.json()['choices'][0]['message']['content'])

# Deep web search for comprehensive research ($0.06 per request)
deep_data = {

```

--------------------------------

### Python SDK Example for Audio Transcription

Source: https://docs.nano-gpt.com/api-reference/endpoint/audio-transcriptions

An example demonstrating how to use the OpenAI Python SDK to interact with the NanoGPT transcription endpoint. It shows how to configure the client and send an audio file for transcription.

```python
from openai import OpenAI

client = OpenAI(
    base_url="https://nano-gpt.com/api/v1",
    api_key="YOUR_API_KEY"
)

with open("audio.mp3", "rb") as audio_file:
    transcript = client.audio.transcriptions.create(
        model="Whisper-Large-V3",
        file=audio_file
    )

print(transcript.text)
```

--------------------------------

### API Key Format and Usage Example

Source: https://docs.nano-gpt.com/authentication

Illustrates the format of NanoGPT API keys and provides an example of how to send the API key in the Authorization header for API requests. It covers both recommended 'sk-nano-' format and legacy UUID keys.

```text
sk-nano-<uuid>
```

```bash
curl "https://nano-gpt.com/api/v1/models" \
  -H "Authorization: Bearer sk-nano-YOUR_API_KEY"
```

--------------------------------

### Install NanoGPT MCP Server Globally

Source: https://docs.nano-gpt.com/integrations/mcp

This command installs the NanoGPT MCP server globally on your system using npm. After installation, you can add it to your MCP client.

```bash
npm install -g @nanogpt/mcp
```

--------------------------------

### NanoGPT CLI Device Login Integration Guide

Source: https://docs.nano-gpt.com/integrations/cli-login

This guide explains how to integrate device login into your CLI application so users can authenticate without embedding a browser. The flow involves starting the login, user approval via a browser, and polling for an API key.

```APIDOC
## POST /api/cli-login/start

### Description
Initiates the CLI device login flow. This endpoint generates a device code and a user code for the user to approve the login via a web interface.

### Method
POST

### Endpoint
/api/cli-login/start

### Parameters
#### Request Body
- **client_name** (string) - Optional - Your application name (max 64 chars). Used to identify the API key in the user's account.

### Request Example
```json
{
  "client_name": "your-app-name"
}
```

### Response
#### Success Response (200)
- **device_code** (string) - Secret token for polling. Keep this secure and do not display to the user.
- **user_code** (string) - Human-readable code displayed on the approval page.
- **verification_uri** (string) - Base URL for the user to visit.
- **verification_uri_complete** (string) - Full URL with the code pre-filled. Display this to the user.
- **expires_in** (integer) - Seconds until the code expires (600 = 10 minutes).
- **interval** (integer) - Recommended polling interval in seconds.

#### Response Example
```json
{
  "device_code": "0Reyai4sGnBE8em8lMLRxDhC-XtQMC2obf8hnVDUWws",
  "user_code": "VS8Q-ZY3Q",
  "verification_uri": "https://nano-gpt.com/cli-login/verify",
  "verification_uri_complete": "https://nano-gpt.com/cli-login/verify?code=VS8Q-ZY3Q",
  "expires_in": 600,
  "interval": 2
}
```

## POST /api/cli-login/poll

### Description
Polls the status of the device login. This endpoint should be called repeatedly with the `device_code` obtained from the `/api/cli-login/start` endpoint until the login is approved or expires.

### Method
POST

### Endpoint
/api/cli-login/poll

### Parameters
#### Request Body
- **device_code** (string) - Required - The secret token obtained from the `/api/cli-login/start` endpoint.

### Request Example
```json
{
  "device_code": "0Reyai4sGnBE8em8lMLRxDhC-XtQMC2obf8hnVDUWws"
}
```

### Response
#### Success Response (200)
- **status** (string) - Indicates the login status. "approved" signifies successful authentication.
- **key** (string) - The generated API key (e.g., `sk-nano-...`) upon successful approval.

#### Success Response (202)
- **status** (string) - "authorization_pending" indicates that the user has not yet approved the login.

#### Error Response (410)
- **status** (string) - "expired" indicates that the device code has expired and the flow needs to be restarted.

#### Error Response (409)
- **status** (string) - "consumed" indicates that the API key has already been delivered.

#### Error Response (404)
- **error** (string) - "invalid_code" indicates that the provided `device_code` is invalid.

#### Response Example (Pending)
```json
{
  "status": "authorization_pending"
}
```

#### Response Example (Approved)
```json
{
  "status": "approved",
  "key": "sk-nano-your-api-key"
}
```

#### Response Example (Expired)
```json
{
  "status": "expired"
}
```
```

--------------------------------

### Simple String Input Example (JSON)

Source: https://docs.nano-gpt.com/api-reference/endpoint/responses

Demonstrates how to provide input to the API using a simple string. This is suitable for straightforward text-based prompts.

```json
{
  "model": "openai/gpt-5.2",
  "input": "What is the capital of France?"
}
```

--------------------------------

### Discover Models and Providers API Response Example

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/provider-selection

Example JSON response from the discover providers endpoint. It includes model details, default pricing, and a list of available providers with their specific pricing and availability.

```JSON
{
  "canonicalId": "model-id",
  "displayName": "Model Name",
  "supportsProviderSelection": true,
  "defaultPrice": { "inputPer1kTokens": 0.0004, "outputPer1kTokens": 0.00175 },
  "providers": [
    {
      "provider": "provider-id",
      "pricing": { "inputPer1kTokens": 0.00042, "outputPer1kTokens": 0.0018375 },
      "available": true
    }
  ]
}
```

--------------------------------

### Generate Image using cURL

Source: https://docs.nano-gpt.com/quickstart

This example demonstrates how to generate an image using the OpenAI-compatible endpoint via a cURL command. It specifies the model, prompt, number of images, and desired size.

```bash
curl https://nano-gpt.com/v1/images/generations \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{ 
    "model": "hidream", 
    "prompt": "A serene landscape at sunset", 
    "n": 1, 
    "size": "1024x1024" 
  }'

```

--------------------------------

### Install Codex CLI using npm

Source: https://docs.nano-gpt.com/integrations/codex-cli

Installs the Codex CLI globally using npm. Requires Node.js 22 or newer. After installation, navigate to your project directory and start Codex.

```bash
npm install -g @openai/codex

cd your-awesome-project

codex
```

--------------------------------

### Launch Grok CLI with Specific Models

Source: https://docs.nano-gpt.com/integrations/grok-cli

Demonstrates how to launch the Grok CLI and specify a model to use. It shows examples for Grok models and other providers accessible through NanoGPT, such as OpenAI and Anthropic.

```bash
# Grok models
grok --model grok-3
grok --model grok-3-fast
grok --model grok-3-mini

# Other providers through NanoGPT
grok --model openai/gpt-5.2
grok --model anthropic/claude-opus-4.5
grok --model google/gemini-3-flash-preview
```

--------------------------------

### NanoGPT Retry Guidance (Text Example)

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/error-handling

Provides a simple text-based example of suggested retry intervals for API requests. This illustrates the concept of exponential backoff, where the delay between retries increases with each attempt.

```text
Attempt 1: 1s
Attempt 2: 2s
Attempt 3: 4s
```

--------------------------------

### Generate Video Content API Endpoint

Source: https://docs.nano-gpt.com/api-reference/endpoint/video-content

This snippet shows how to use `curl` to make a GET request to the `/api/generate-video/content` endpoint. It demonstrates how to pass query parameters like `runId`, `model`, and `variant`, and includes an example of how to pipe the output to `jq` for pretty-printing JSON. Requires an API key for authentication.

```bash
curl -s "https://nano-gpt.com/api/generate-video/content?runId=RUN_ID&model=sora-2&variant=video" \
  -H "x-api-key: YOUR_API_KEY" | jq .
```

--------------------------------

### Install Droid CLI

Source: https://docs.nano-gpt.com/integrations/droid

Installs the Droid Command Line Interface (CLI) agent. This command fetches the installation script and executes it.

```bash
curl -fsSL https://app.factory.ai/cli | sh
```

--------------------------------

### Install NanoGPTJS Node.js Library

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/javascript

Installs the NanoGPTJS library using npm. This is the first step to using the library in your Node.js project.

```bash
npm install nanogptjs
```

--------------------------------

### Fetch NanoGPT Models (Bash)

Source: https://docs.nano-gpt.com/api-reference/endpoint/models

Fetches a list of available NanoGPT models using a simple GET request. This is the most basic way to query the API.

```bash
curl "https://nano-gpt.com/api/v1/models"
```

--------------------------------

### Install NanoGPT-client using npm or yarn

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/typescript

Instructions for installing the NanoGPT-client library using either npm or yarn package managers. This is the first step to integrating the client into your TypeScript project.

```bash
npm install nanogpt-client
```

```bash
yarn add nanogpt-client
```

--------------------------------

### Output Type Selection Guide

Source: https://docs.nano-gpt.com/api-reference/endpoint/web-search

A guide to selecting the appropriate `outputType` for different use cases, explaining the benefits of `searchResults`, `sourcedAnswer`, and `structured`.

```APIDOC
## Output Type Selection Guide

- **searchResults**: Best for general searches where you want to see multiple sources and snippets. Ideal for research and exploration.
- **sourcedAnswer**: Best when you want a comprehensive answer synthesized from multiple sources. Great for factual questions and summaries.
- **structured**: Best when you need to extract specific data points in a predictable format. Perfect for data collection and automation.
```

--------------------------------

### Start Codex CLI with a Specific Model

Source: https://docs.nano-gpt.com/integrations/codex-cli

Starts the Codex CLI and specifies a particular model to use, in this case, 'openai/gpt-5.2'.

```bash
codex --model openai/gpt-5.2
```

--------------------------------

### NANO/USD Price Response Example

Source: https://docs.nano-gpt.com/api-reference/endpoint/crypto-deposits

Example JSON response for the `/api/get-nano-price` endpoint, indicating the trading pair and the latest price. This helps in understanding the structure of the data returned for NANO pricing.

```json
{
  "pair": "NANOUSD",
  "latestPrice": 1.23
}
```

--------------------------------

### Detailed Model Object Structure Example (JSON)

Source: https://docs.nano-gpt.com/api-reference/endpoint/personalized-models

This JSON object provides an example of a detailed model entry, including additional fields such as 'name', 'description', 'context_length', 'capabilities', 'pricing', 'icon_url', and 'cost_estimate'.

```json
{
  "id": "openai/gpt-5.2",
  "object": "model",
  "created": 1736966400,
  "owned_by": "openai",
  "name": "GPT-5.2",
  "description": "OpenAI flagship general-purpose model",
  "context_length": 128000,
  "capabilities": { "vision": false },
  "pricing": {
    "prompt": 2.50,
    "completion": 10.00,
    "currency": "USD",
    "unit": "per_million_tokens"
  },
  "icon_url": "/icons/OpenAI.svg",
  "cost_estimate": {
    "cheap": false
  }
}
```

--------------------------------

### Web Search Examples (Python, JavaScript, cURL)

Source: https://docs.nano-gpt.com/api-reference/endpoint/web-search

Demonstrates basic and advanced web search queries using the Nano-GPT API. Includes examples for Python, JavaScript, and cURL, showcasing different search parameters like provider, depth, date filtering, and output types.

```python
import requests
import json

# Your API key
api_key = "YOUR_API_KEY"

# API endpoint
url = "https://nano-gpt.com/api/web"

# Headers
headers = {
    "Content-Type": "application/json",
    "x-api-key": api_key
}

# Basic search
basic_search = {
    "query": "artificial intelligence trends 2025",
    "provider": "linkup"
}

response = requests.post(url, headers=headers, json=basic_search)
results = response.json()

# Print results
if results["metadata"]["outputType"] == "searchResults":
    for result in results["data"]:
        print(f"Title: {result['title']}")
        print(f"URL: {result['url']}")
        print(f"Snippet: {result.get('snippet', 'N/A')[:200]}...")
        print("-" * 50)
    print(f"Search cost: ${results['metadata']['cost']}")
```

```javascript
const axios = require('axios');

// Your API key
const apiKey = 'YOUR_API_KEY';

// API endpoint
const url = 'https://nano-gpt.com/api/web';

// Search with structured output
async function searchWithStructuredOutput() {
  const searchData = {
    query: 'top tech companies by revenue',
    provider: 'linkup',
    outputType: 'structured',
    structuredOutputSchema: JSON.stringify({
      type: 'object',
      properties: {
        companies: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              name: { type: 'string' },
              revenue: { type: 'string' },
              year: { type: 'string' }
            }
          }
        }
      })
    };

    try {
      const response = await axios.post(url, searchData, {
        headers: {
          'Content-Type': 'application/json',
          'x-api-key': apiKey
        }
      });

      console.log('Structured Results:', JSON.stringify(response.data.data, null, 2));
      console.log('Search Cost:', response.data.metadata.cost);
    } catch (error) {
      console.error('Error:', error.response?.data || error.message);
    }
  }

  searchWithStructuredOutput();
```

```bash
# Basic search
curl -X POST https://nano-gpt.com/api/web \
    -H "Content-Type: application/json" \
    -H "x-api-key: YOUR_API_KEY" \
    -d '{
      "query": "latest AI news",
      "provider": "linkup"
    }'

# Deep search with date filtering
curl -X POST https://nano-gpt.com/api/web \
    -H "Content-Type: application/json" \
    -H "x-api-key: YOUR_API_KEY" \
    -d '{
      "query": "climate change research",
      "provider": "tavily",
      "depth": "deep",
      "fromDate": "2025-01-01",
      "toDate": "2025-07-01"
    }'

# Sourced answer with domain filtering
curl -X POST https://nano-gpt.com/api/web \
    -H "Content-Type: application/json" \
    -H "x-api-key: YOUR_API_KEY" \
    -d '{
      "query": "Microsoft quarterly earnings",
      "provider": "exa",
      "outputType": "sourcedAnswer",
      "includeDomains": ["microsoft.com", "reuters.com", "bloomberg.com"]
    }'
```

--------------------------------

### Text-to-Speech API Usage (Python, JavaScript, cURL)

Source: https://docs.nano-gpt.com/api-reference/endpoint/tts

Demonstrates how to convert text to speech using the nano-gpt API. Supports various models and customization options. The Python and JavaScript examples handle both JSON responses with audio URLs and direct binary audio data, while the cURL example shows a basic POST request.

```python
import requests

def text_to_speech(text, model="Kokoro-82m", voice=None, **kwargs):
    headers = {
        "x-api-key": "YOUR_API_KEY",
        "Content-Type": "application/json"
    }
    
    payload = {
        "text": text,
        "model": model
    }
    
    if voice:
        payload["voice"] = voice
    
payload.update(kwargs)
    
    response = requests.post(
        "https://nano-gpt.com/api/tts",
        headers=headers,
        json=payload
    )
    
    if response.status_code == 200:
        content_type = response.headers.get('content-type', '')
        
        if 'application/json' in content_type:
            # JSON response with audio URL
            data = response.json()
            audio_response = requests.get(data['audioUrl'])
            with open('output.wav', 'wb') as f:
                f.write(audio_response.content)
        else:
            # Binary audio data (OpenAI models)
            with open('output.mp3', 'wb') as f:
                f.write(response.content)
        
        return response
    else:
        raise Exception(f"Error: {response.status_code}")

# Basic usage
text_to_speech(
    "Hello! Welcome to our service.",
    model="Kokoro-82m",
    voice="af_bella"
)
```

```javascript
async function textToSpeech(text, options = {}) {
    const payload = {
        text: text,
        model: options.model || 'Kokoro-82m',
        ...options
    };
    
    const response = await fetch('https://nano-gpt.com/api/tts', {
        method: 'POST',
        headers: {
            'x-api-key': 'YOUR_API_KEY',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    });
    
    if (response.ok) {
        const contentType = response.headers.get('content-type');
        
        if (contentType.includes('application/json')) {
            const data = await response.json();
            console.log('Audio URL:', data.audioUrl);
            return data;
        } else {
            // Binary audio data
            const audioBlob = await response.blob();
            const url = URL.createObjectURL(audioBlob);
            console.log('Audio blob URL:', url);
            return { audioBlob, url };
        }
    } else {
        throw new Error(`Error: ${response.status}`);
    }
}

// Usage
textToSpeech('Hello world!', {
    model: 'Kokoro-82m',
    voice: 'af_bella',
    speed: 1.1
});
```

```bash
curl -X POST https://nano-gpt.com/api/tts \
  -H "x-api-key: YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "text": "Hello! Welcome to our service.",
    "model": "Kokoro-82m",
    "voice": "af_bella",
    "speed": 1.0
  }'
```

--------------------------------

### Install Gemini CLI OpenRouter Fork (Bash)

Source: https://docs.nano-gpt.com/integrations/gemini-cli

Clones the community-maintained fork of Gemini CLI that supports OpenRouter, installs dependencies, and checks out the correct branch. Requires Node.js 18 or newer.

```bash
git clone https://github.com/heartyguy/gemini-cli
cd gemini-cli
git checkout feature/openrouter-support
npm install
```

--------------------------------

### Successful API Response Example (JSON)

Source: https://docs.nano-gpt.com/api-reference/endpoint/responses

A comprehensive example of a successful response from the Nano-GPT API. It includes details about the request, processing status, model information, and the final output content.

```json
{
  "id": "resp_abc123",
  "object": "response",
  "created_at": 1699000000,
  "completed_at": 1699000001,
  "model": "openai/gpt-5.2",
  "status": "completed",
  "instructions": null,
  "previous_response_id": null,
  "tools": [],
  "tool_choice": "auto",
  "parallel_tool_calls": false,
  "truncation": "disabled",
  "text": {
    "format": { "type": "text" },
    "verbosity": "medium"
  },
  "reasoning": null,
  "temperature": 1,
  "top_p": 1,
  "presence_penalty": 0,
  "frequency_penalty": 0,
  "top_logprobs": 0,
  "max_output_tokens": null,
  "max_tool_calls": null,
  "user": null,
  "store": true,
  "background": false,
  "safety_identifier": null,
  "prompt_cache_key": null,
  "output": [
    {
      "type": "message",
      "id": "msg_xyz789",
      "role": "assistant",
      "status": "completed",
      "content": [
        {
          "type": "output_text",
          "text": "The capital of France is Paris.",
          "annotations": [],
          "logprobs": []
        }
      ]
    }
  ],
  "output_text": "The capital of France is Paris.",
  "usage": {
    "input_tokens": 15,
    "output_tokens": 10,
    "total_tokens": 25,
    "input_tokens_details": { "cached_tokens": 0 },
    "output_tokens_details": { "reasoning_tokens": 0 }
  },
  "metadata": {},
  "service_tier": "auto"
}
```

--------------------------------

### GET /v1/video-models

Source: https://docs.nano-gpt.com/api-reference/endpoint/models

Lists all available video generation models. The `detailed` query parameter can be used to include pricing, capabilities, and supported parameters.

```APIDOC
## GET /v1/video-models

### Description
Lists all available video generation models. The `detailed` query parameter can be used to include pricing, capabilities, and supported parameters.

### Method
GET

### Endpoint
/v1/video-models

### Query Parameters
- **detailed** (boolean) - Optional - Include pricing, capabilities, and supported parameters. Defaults to `true`.

### Response
#### Success Response (200)
- **object** (string) - The type of the response object, typically "list".
- **data** (array) - An array of video model objects.
- **meta** (object) - Metadata about the list, including count and generation timestamp.

Each model object (when `detailed=true`):
- **id** (string) - Unique identifier for the model.
- **object** (string) - The type of the object, typically "model".
- **created** (integer) - Timestamp of model creation.
- **owned_by** (string) - The entity that owns the model.
- **name** (string) - The display name of the model.
- **description** (string) - A description of the model.
- **architecture** (object) - Details about the model's architecture, including modalities.
- **pricing** (object) - Pricing information for the model. The shape varies by model.
- **capabilities** (object) - Supported capabilities of the model.
- **settings** (array) - Settings specific to the model.
- **icon_url** (string) - URL for the model's icon.
- **label** (string) - A label for the model.
- **tags** (array) - Tags associated with the model.
- **category** (string) - The category of the model (e.g., "video").

### Response Example
```json
{
  "object": "list",
  "data": [ ... ],
  "meta": {
    "count": 5,
    "generated_at": "2025-01-28T12:00:00.000Z"
  }
}
```

### Notes
- With `detailed=false`, the response includes only `id`, `object`, `created`, and `owned_by` per model.
- Video pricing structures vary by model. The `pricing` object shape depends on the specific model.
- Pricing values are in USD.
```

--------------------------------

### Array Input Example (JSON)

Source: https://docs.nano-gpt.com/api-reference/endpoint/responses

Illustrates providing input to the API as an array of message objects. This format allows for more structured conversations with distinct roles (e.g., user, assistant).

```json
{
  "model": "openai/gpt-5.2",
  "input": [
    {
      "type": "message",
      "role": "user",
      "content": "What is the capital of France?"
    }
  ]
}
```

--------------------------------

### Vision Example with Anthropic SDK (Node.js)

Source: https://docs.nano-gpt.com/api-reference/endpoint/messages

Demonstrates how to use the Anthropic SDK in Node.js to process images. It involves reading image data from a file and including it in the message content.

```typescript
import Anthropic from "@anthropic-ai/sdk";
import fs from "fs";

const anthropic = new Anthropic({
  apiKey: process.env.NANOGPT_API_KEY,
  baseURL: "https://nano-gpt.com/api"
});

const imageData = fs.readFileSync("image.jpg").toString("base64");

const message = await anthropic.messages.create({
  model: "claude-opus-4-5-20251101",
  max_tokens: 1024,
  messages: [
    {
      role: "user",
      content: [
        { type: "text", text: "What's in this image?" },
        {
          type: "image",
          source: {
            type: "base64",
            media_type: "image/jpeg",
            data: imageData
          }
        }
      ]
    }
  ]
});
```

--------------------------------

### Install Claude Code with npm

Source: https://docs.nano-gpt.com/integrations/claude-code

Installs the Claude Code package globally using npm. Requires Node.js 18 or newer. After installation, navigate to your project directory and start Claude Code.

```bash
npm install -g @anthropic-ai/claude-code

cd your-awesome-project

claude
```

--------------------------------

### Fetch Detailed Subscription-Only NanoGPT Models with API Key (Bash)

Source: https://docs.nano-gpt.com/api-reference/endpoint/models

Fetches a detailed list of subscription-only NanoGPT models, including pricing, using an API key for authentication. Assumes the NANOGPT_API_KEY environment variable is set.

```bash
curl -H "x-api-key: $NANOGPT_API_KEY" \
  "https://nano-gpt.com/api/subscription/v1/models?detailed=true"
```

--------------------------------

### Persistent Provider Preferences - GET Endpoint

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/provider-selection

Example JSON response for the GET /api/user/provider-preferences endpoint. It outlines the user's saved provider preferences, including preferred and excluded providers, fallback settings, and model-specific overrides.

```JSON
{
  "preferredProviders": ["provider-a", "provider-b"],
  "excludedProviders": ["provider-c"],
  "enableFallback": true,
  "modelOverrides": {
    "model-id": {
      "preferredProviders": ["provider-b"],
      "enableFallback": false
    }
  },
  "availableProviders": ["provider-a", "provider-b", "provider-c"]
}
```

--------------------------------

### Example response.output_text.delta JSON

Source: https://docs.nano-gpt.com/api-reference/endpoint/responses

Demonstrates the structure of a delta output text event, including item_id, sequence_number, and incremental content.

```json
{
  "type": "response.output_text.delta",
  "item_id": "msg_...",
  "output_index": 0,
  "content_index": 0,
  "delta": "Hello",
  "logprobs": [...],
  "sequence_number": 5
}
```

--------------------------------

### X-402 Payment Status Response Example

Source: https://docs.nano-gpt.com/api-reference/miscellaneous/x402

An example JSON response for the 'Get Payment Status' API endpoint. It details the payment ID, status, amounts, payment address, and expiration time. This structure helps clients understand the state of their micropayment.

```JSON
{
  "paymentId": "pay_abc123def456...",
  "status": "pending",
  "amountRequired": "0.05",
  "amountReceived": "0",
  "amountRemaining": "0.05",
  "payTo": "nano_1abc...",
  "expiresAt": 1700000900,
  "pollAfterSeconds": 2,
  "readyToComplete": false
}
```

--------------------------------

### Poll for Completion HTTP Request

Source: https://docs.nano-gpt.com/api-reference/endpoint/responses

An example HTTP GET request to poll for the status of a background job using its response ID.

```http
GET /v1/responses/resp_abc123
Authorization: Bearer YOUR_API_KEY
```