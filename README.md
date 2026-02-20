# nano-zed

`nano-zed` is a community fork of [Zed](https://github.com/zed-industries/zed), focused on a NanoGPT-first agent experience.

## What Changed In This Fork

- Rebranded app surfaces to `nano-zed`.
- Native built-in agent defaults to the `nanogpt` provider.
- Default agent model is `minimax/minimax-m2.5`.
- NanoGPT models are loaded dynamically from NanoGPT's models API.
- Optional provider selection is supported for models that expose provider options.
- NanoGPT MCP server is configured by default (`@nanogpt/mcp`).
- On startup, `nano-zed` prompts for a NanoGPT API key when needed.

## Download

Download binaries only from this repository's Releases page:

- https://github.com/nanogpt-community/nano-zed/releases

## First Launch

- If you have not configured a NanoGPT key yet, `nano-zed` will prompt you.
- You can also set `NANOGPT_API_KEY` in your environment.

## Build From Source

- macOS: `docs/src/development/macos.md`
- Linux: `docs/src/development/linux.md`
- Windows: `docs/src/development/windows.md`

## Upstream

This project is based on Zed and keeps large parts of the upstream codebase.

- Upstream repository: https://github.com/zed-industries/zed

## License

See `LICENSE-AGPL` and `LICENSE-APACHE`.
