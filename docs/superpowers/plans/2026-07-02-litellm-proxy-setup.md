# LiteLLM Proxy Setup Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Set up a dockerized LiteLLM proxy rotating fallback API keys from `llm-proxy` for NVIDIA NIM, Gemini, Cerebras, OpenRouter, and Opencode-Zen, and configure Codex CLI to use it.

**Architecture:** A local LiteLLM proxy running on port `4000` via Docker Compose using official LiteLLM image with a static configuration for routing and load-balancing.

**Tech Stack:** Docker, Docker Compose, LiteLLM, YAML, TOML

## Global Constraints
- Target directory: `/home/ishanp/docker/litellm-proxy/`
- Target Codex configuration: `/home/ishanp/.codex/config.toml`
- LiteLLM version: `ghcr.io/berriai/litellm:main-latest`

---

### Task 1: Replicate `.env` Configuration
**Files:**
- Create: `/home/ishanp/docker/litellm-proxy/.env`

- [ ] **Step 1: Create target directory**
  Run: `mkdir -p /home/ishanp/docker/litellm-proxy/`

- [ ] **Step 2: Copy the .env content**
  Read contents from `/home/ishanp/Documents/GitHub/CLONED-REPOS/LLM-API-Key-Proxy/.env` and write them to `/home/ishanp/docker/litellm-proxy/.env`.
  Ensure `PROXY_API_KEY="your-secure-proxy-key-change-me"` is preserved.

- [ ] **Step 3: Verify the file exists**
  Run: `ls -la /home/ishanp/docker/litellm-proxy/.env`
  Expected: File exists and has readable permissions.

---

### Task 2: Create LiteLLM Configuration File
**Files:**
- Create: `/home/ishanp/docker/litellm-proxy/litellm_config.yaml`

- [ ] **Step 1: Write `litellm_config.yaml`**
  Create the LiteLLM config routing traffic and specifying the load-balanced API keys.
  
  File content to write:
  ```yaml
  model_list:
    # 1. NVIDIA NIM (minimax-m3)
    - model_name: minimax-m3
      litellm_params:
        model: nvidia_nim/minimax/minimax-m3
        api_key: os.environ/NVIDIA_NIM_API_KEY_1
    - model_name: minimax-m3
      litellm_params:
        model: nvidia_nim/minimax/minimax-m3
        api_key: os.environ/NVIDIA_NIM_API_KEY_2
    - model_name: minimax-m3
      litellm_params:
        model: nvidia_nim/minimax/minimax-m3
        api_key: os.environ/NVIDIA_NIM_API_KEY_3
    - model_name: minimax-m3
      litellm_params:
        model: nvidia_nim/minimax/minimax-m3
        api_key: os.environ/NVIDIA_NIM_API_KEY_4

    # 2. NVIDIA NIM Wildcard
    - model_name: nvidia_nim/*
      litellm_params:
        model: nvidia_nim/*
        api_key: os.environ/NVIDIA_NIM_API_KEY_1
    - model_name: nvidia_nim/*
      litellm_params:
        model: nvidia_nim/*
        api_key: os.environ/NVIDIA_NIM_API_KEY_2
    - model_name: nvidia_nim/*
      litellm_params:
        model: nvidia_nim/*
        api_key: os.environ/NVIDIA_NIM_API_KEY_3
    - model_name: nvidia_nim/*
      litellm_params:
        model: nvidia_nim/*
        api_key: os.environ/NVIDIA_NIM_API_KEY_4

    # 3. Gemini Wildcard
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_1
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_2
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_3
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_4
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_5
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_6
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_7
    - model_name: gemini/*
      litellm_params:
        model: gemini/*
        api_key: os.environ/GEMINI_API_KEY_8

    # 4. Cerebras Wildcard
    - model_name: cerebras/*
      litellm_params:
        model: cerebras/*
        api_key: os.environ/CEREBRAS_API_KEY_1
    - model_name: cerebras/*
      litellm_params:
        model: cerebras/*
        api_key: os.environ/CEREBRAS_API_KEY_2
    - model_name: cerebras/*
      litellm_params:
        model: cerebras/*
        api_key: os.environ/CEREBRAS_API_KEY_3
    - model_name: cerebras/*
      litellm_params:
        model: cerebras/*
        api_key: os.environ/CEREBRAS_API_KEY_4
    - model_name: cerebras/*
      litellm_params:
        model: cerebras/*
        api_key: os.environ/CEREBRAS_API_KEY_5
    - model_name: cerebras/*
      litellm_params:
        model: cerebras/*
        api_key: os.environ/CEREBRAS_API_KEY_6
    - model_name: cerebras/*
      litellm_params:
        model: cerebras/*
        api_key: os.environ/CEREBRAS_API_KEY_7

    # 5. OpenRouter Wildcard
    - model_name: openrouter/*
      litellm_params:
        model: openrouter/*
        api_key: os.environ/OPENROUTER_API_KEY_1
    - model_name: openrouter/*
      litellm_params:
        model: openrouter/*
        api_key: os.environ/OPENROUTER_API_KEY_2
    - model_name: openrouter/*
      litellm_params:
        model: openrouter/*
        api_key: os.environ/OPENROUTER_API_KEY_3
    - model_name: openrouter/*
      litellm_params:
        model: openrouter/*
        api_key: os.environ/OPENROUTER_API_KEY_4
    - model_name: openrouter/*
      litellm_params:
        model: openrouter/*
        api_key: os.environ/OPENROUTER_API_KEY_5
    - model_name: openrouter/*
      litellm_params:
        model: openrouter/*
        api_key: os.environ/OPENROUTER_API_KEY_6
    - model_name: openrouter/*
      litellm_params:
        model: openrouter/*
        api_key: os.environ/OPENROUTER_API_KEY_7

    # 6. Opencode-Zen (Custom OpenAI-Compatible)
    - model_name: opencode-zen/*
      litellm_params:
        model: openai/*
        api_base: https://opencode.ai/zen/v1
        api_key: os.environ/OPENCODE_ZEN_API_KEY_1
    - model_name: opencode-zen/*
      litellm_params:
        model: openai/*
        api_base: https://opencode.ai/zen/v1
        api_key: os.environ/OPENCODE_ZEN_API_KEY_2
    - model_name: opencode-zen/*
      litellm_params:
        model: openai/*
        api_base: https://opencode.ai/zen/v1
        api_key: os.environ/OPENCODE_ZEN_API_KEY_3
    - model_name: opencode-zen/*
      litellm_params:
        model: openai/*
        api_base: https://opencode.ai/zen/v1
        api_key: os.environ/OPENCODE_ZEN_API_KEY_4
    - model_name: opencode-zen/*
      litellm_params:
        model: openai/*
        api_base: https://opencode.ai/zen/v1
        api_key: os.environ/OPENCODE_ZEN_API_KEY_5
    - model_name: opencode-zen/*
      litellm_params:
        model: openai/*
        api_base: https://opencode.ai/zen/v1
        api_key: os.environ/OPENCODE_ZEN_API_KEY_6
    - model_name: opencode-zen/*
      litellm_params:
        model: openai/*
        api_base: https://opencode.ai/zen/v1
        api_key: os.environ/OPENCODE_ZEN_API_KEY_7

  router_settings:
    routing_strategy: simple-shuffle

  general_settings:
    master_key: your-secure-proxy-key-change-me
  ```

- [ ] **Step 2: Verify yaml syntax**
  Check that the written YAML file is correct.

---

### Task 3: Create Docker Compose Configuration
**Files:**
- Create: `/home/ishanp/docker/litellm-proxy/docker-compose.yml`

- [ ] **Step 1: Write `docker-compose.yml`**
  Write the compose configuration mapping ports and configs.
  
  File content to write:
  ```yaml
  services:
    litellm:
      image: ghcr.io/berriai/litellm:main-latest
      container_name: litellm-proxy
      restart: unless-stopped
      ports:
        - "4000:4000"
      volumes:
        - ./litellm_config.yaml:/app/config.yaml:ro
      env_file:
        - .env
      environment:
        - LITELLM_MASTER_KEY=${PROXY_API_KEY}
      command: ["--config", "/app/config.yaml", "--port", "4000"]
  ```

- [ ] **Step 2: Verify compile and compose syntax**
  Run: `docker compose -f /home/ishanp/docker/litellm-proxy/docker-compose.yml config`
  Expected: Valid docker-compose structure output.

---

### Task 4: Start and Verify LiteLLM Container
**Files:**
- None

- [ ] **Step 1: Start container**
  Run: `docker compose -f /home/ishanp/docker/litellm-proxy/docker-compose.yml up -d`
  Expected: Containers start up and report `Running` status.

- [ ] **Step 2: Inspect LiteLLM logs**
  Run: `docker logs litellm-proxy`
  Expected: Log output showing server running on `http://0.0.0.0:4000`.

- [ ] **Step 3: Test connection with curl**
  Run:
  ```bash
  curl -X POST http://localhost:4000/v1/chat/completions \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer your-secure-proxy-key-change-me" \
    -d '{
      "model": "minimax-m3",
      "messages": [{"role": "user", "content": "test connection"}]
    }'
  ```
  Expected: Valid API response containing completion output.

---

### Task 5: Configure Codex CLI to Target LiteLLM Proxy
**Files:**
- Modify: `/home/ishanp/.codex/config.toml`

- [ ] **Step 1: Write the updated `/home/ishanp/.codex/config.toml`**
  Modify `/home/ishanp/.codex/config.toml` to:
  ```toml
  model = "minimax-m3"
  model_provider = "nvidia-nim"

  [model_providers.nvidia-nim]
  name = "NVIDIA NIM (via LiteLLM)"
  base_url = "http://localhost:4000"
  env_key = "NVIDIA_API_KEY"
  wire_api = "responses"

  [shell_environment_policy]
  inherit = "all"
  ignore_default_excludes = true
  set = { NVIDIA_API_KEY = "your-secure-proxy-key-change-me" }

  [projects."/home/ishanp/Documents/GitHub/MY-PROJECTS/MCP-AND-CLIS/slideforge-rust"]
  trust_level = "trusted"

  [tui.model_availability_nux]
  "gpt-5.5" = 4
  ```

- [ ] **Step 2: Run Codex verification command**
  Run: `codex "Hello, are you MiniMax M3?"`
  Expected: Correct response from the model.
