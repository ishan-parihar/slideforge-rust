# LiteLLM Proxy Setup Design Specification

**Date**: 2026-07-02  
**Author**: Antigravity  
**Status**: Approved

## 1. Objective
To set up LiteLLM as a local proxy on port `4000` to route and load-balance LLM requests from Codex CLI to NVIDIA NIM (MiniMax M3), Gemini, Cerebras, OpenRouter, and Opencode-Zen APIs. This setup replicates the API rotation resilience of `llm-proxy` using LiteLLM's native load balancing capabilities.

## 2. Directory and File Layout
All configuration and deployment files will reside in:
`/home/ishanp/docker/litellm-proxy/`

### 2.1 Files Created
- `/home/ishanp/docker/litellm-proxy/.env`: Local environment file with all API keys.
- `/home/ishanp/docker/litellm-proxy/litellm_config.yaml`: LiteLLM router and model list mapping.
- `/home/ishanp/docker/litellm-proxy/docker-compose.yml`: Container orchestrator for LiteLLM.

### 2.2 Files Modified
- `/home/ishanp/.codex/config.toml`: Updated to target the new LiteLLM proxy at `http://localhost:4000`.

## 3. Configuration Details

### 3.1 Environment File (`.env`)
The `.env` file is replicated from `/home/ishanp/Documents/GitHub/CLONED-REPOS/LLM-API-Key-Proxy/.env` with the following variables:
- `PROXY_API_KEY`: Set to `"your-secure-proxy-key-change-me"`
- `GEMINI_API_KEY_1` through `8`
- `CEREBRAS_API_KEY_1` through `7`
- `OPENROUTER_API_KEY_1` through `7`
- `OPENCODE_ZEN_API_KEY_1` through `7`
- `NVIDIA_NIM_API_KEY_1` through `4`

### 3.2 LiteLLM Configuration (`litellm_config.yaml`)
Configures the routing strategy as `simple-shuffle` and maps models:
- `minimax-m3` -> load balances across `NVIDIA_NIM_API_KEY_1` through `4` targeting the `nvidia_nim/minimax/minimax-m3` model.
- `nvidia_nim/*` -> load balances across `NVIDIA_NIM_API_KEY_1` through `4`.
- `gemini/*` -> load balances across `GEMINI_API_KEY_1` through `8` targeting the `gemini/*` API.
- `cerebras/*` -> load balances across `CEREBRAS_API_KEY_1` through `7` targeting the `cerebras/*` API.
- `openrouter/*` -> load balances across `OPENROUTER_API_KEY_1` through `7` targeting the `openrouter/*` API.
- `opencode-zen/*` -> load balances across `OPENCODE_ZEN_API_KEY_1` through `7` targeting the `openai/*` API with `api_base: https://opencode.ai/zen/v1`.

### 3.3 Docker Compose (`docker-compose.yml`)
Runs the official `ghcr.io/berriai/litellm:main-latest` container, mounting `litellm_config.yaml` and passing the `.env` variables.

## 4. Verification Plan
1. Start the Docker container: `docker compose up -d`.
2. Inspect LiteLLM logs: `docker logs litellm-proxy`.
3. Test Codex connection using `codex "Hello, are you MiniMax M3?"`.
