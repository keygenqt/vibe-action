# Configuration

Vibe Action uses a single YAML config file at `~/.vibe-action/config.yaml`. It is created automatically on first run.

## Action

Runtime settings for all flows.

```yaml
action:
  system: |
    You are Vibe Action — a CLI tool, not a chatbot.
    Work fast. Don't think too much. Just do the task.
    Output ONLY the requested result.
  retries: 2
```

| Field     | Type    | Description                                      |
| --------- | ------- | ------------------------------------------------ |
| `system`  | string  | Global system prompt for all LLM requests        |
| `retries` | integer | Number of retries for failed LLM steps (0 = off) |

## Cluster

Define one or more LLM providers. The engine sends prompts to all nodes in parallel.
Use `role` to assign models to specific complexity levels.

```yaml
cluster:
  - provider: ollama
    host: http://localhost:11434
    model: qwen2.5-coder:14b-instruct
    role: medium
    timeout_secs: 60
    temperature: 0.1
    seed: 42
    num_ctx: 4096
    num_predict: 2048
    parallel: 1
```

### Cluster Node Fields

| Field          | Type    | Description                            |
| -------------- | ------- | -------------------------------------- |
| `provider`     | string  | `ollama`, `deepseek`, `qwen`           |
| `host`         | string  | API endpoint URL                       |
| `model`        | string  | Model name                             |
| `role`         | string  | `small`, `medium`, `large` (optional)  |
| `timeout_secs` | integer | Request timeout in seconds             |
| `temperature`  | float   | 0.0-2.0, lower = more deterministic    |
| `seed`         | integer | Random seed for reproducibility        |
| `num_ctx`      | integer | Context window size in tokens          |
| `num_predict`  | integer | Max tokens to generate                 |
| `api_key`      | string  | API key for cloud providers (optional) |
| `parallel`     | integer | Concurrent connections (default: 1)    |

### Multi-Node Cluster with Roles

```yaml
cluster:
  - provider: ollama
    host: http://localhost:11434
    model: qwen2.5-coder:3b-instruct
    role: small
    timeout_secs: 30
    temperature: 0.0
    seed: 42
    num_ctx: 4096
    num_predict: 512
    parallel: 2

  - provider: ollama
    host: http://192.168.1.10:11434
    model: qwen2.5-coder:14b-instruct
    role: medium
    timeout_secs: 60
    temperature: 0.1
    seed: 42
    num_ctx: 8192
    num_predict: 2048
    parallel: 1

  - provider: deepseek
    host: https://api.deepseek.com/v1
    model: deepseek-v4-flash
    role: large
    timeout_secs: 120
    temperature: 0.1
    seed: 42
    num_ctx: 16384
    num_predict: 8192
    api_key: sk-...
    parallel: 2
```

Nodes with `role: small` are used for `run: llm_small`, `role: medium` for `llm_medium`, `role: large` for `llm_large`.
Nodes without a `role` respond to all requests.

## Actions Directory

Actions are stored in `~/.vibe-action/actions/`. The directory is created on first run with default actions. Override with `VIBE_ACTION_PATH`.

```
~/.vibe-action/
├── config.yaml
└── actions/
    ├── commit.yaml
    ├── extract.yaml
    ├── find.yaml
    ├── mock.yaml
    ├── naming.yaml
    ├── regex.yaml
    ├── spellcheck.yaml
    ├── synonyms.yaml
    ├── tone.yaml
    └── translate.yaml
```

Add your own `.yaml` files here — they will be loaded automatically.
