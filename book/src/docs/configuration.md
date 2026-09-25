# Configuration

Vibe Action uses a single YAML config file at `~/.vibe-action/config.yaml`.
Created automatically on first run. Override the path with `VIBE_CONFIG`.

## Config file structure

```yaml
version: '0.0.3'

action:
  system: |-
    You are Vibe Action — a CLI tool, not a chatbot.
    Work fast. Just do the task.
    Output ONLY the requested result.
  retries: 2

cluster:
  - provider: ollama
    host: http://localhost:11434
    model: qwen2.5-coder:3b-instruct
    role: small
    timeout_secs: 30
    temperature: 0.1
    seed: 42
    num_ctx: 4096
    num_predict: 2048
    parallel: 1
```

## Version

Top-level `version` tracks the config schema. The engine compares it with
its internal `CONFIG_VERSION` on startup — a mismatch is a hard error.

Fix: bump `version` in your config, or delete `~/.vibe-action/config.yaml`
and restart (fresh defaults are written).

## Action settings

| Field     | Type    | Default      | Description                            |
| --------- | ------- | ------------ | -------------------------------------- |
| `system`  | string  | _(see code)_ | Global system prompt for all LLM calls |
| `retries` | integer | `2`          | Retries for failed LLM steps (0 = off) |

The default system prompt instructs the model to be brief and output only
the requested result — no explanations, no Markdown fences unless asked.

## Cluster

Define one or more LLM provider nodes. The engine routes each pipeline
step to nodes whose `role` matches the step's `run` size
(see [YAML Format](./pipeline-yaml.md)).

### Cluster node fields

| Field          | Type    | Required | Description                                   |
| -------------- | ------- | -------- | --------------------------------------------- |
| `provider`     | string  | yes      | `ollama`, `deepseek`, `qwen`, `kimi`, `zhipu` |
| `host`         | string  | yes      | API endpoint URL                              |
| `model`        | string  | yes      | Model name                                    |
| `role`         | string  | no       | `tiny`, `small`, `medium`, `large`, `vision`  |
| `timeout_secs` | integer | yes      | Request timeout in seconds                    |
| `temperature`  | float   | yes      | 0.0–2.0, lower = more deterministic           |
| `seed`         | integer | yes      | Random seed for reproducibility               |
| `num_ctx`      | integer | yes      | Context window size in tokens                 |
| `num_predict`  | integer | yes      | Max tokens to generate                        |
| `api_key`      | string  | no       | API key for cloud providers                   |
| `parallel`     | integer | yes      | Concurrent connections (default: 1)           |

### Role-based routing

Each pipeline action declares a `run` size. The engine filters cluster
nodes by matching `role`:

| `run` value | Matches `role` |
| ----------- | -------------- |
| `tiny`      | `tiny`         |
| `small`     | `small`        |
| `medium`    | `medium`       |
| `large`     | `large`        |
| `vision`    | `vision`       |

- No node with the requested role → **fallback**: all nodes are used.
- Node without a `role` → responds to every request.
- Missing role at pipeline start → warning printed.

### Multi-node example

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
  - provider: ollama
    host: http://localhost:11434
    model: qwen2.5vl:7b
    role: vision
    timeout_secs: 120
    temperature: 0.4
    seed: 42
    num_ctx: 8192
    num_predict: 8192
    parallel: 1
```

## File layout

```text
~/.vibe-action/
├── config.yaml
└── actions/
    ├── comment.yaml
    ├── commit.yaml
    └── ...
```

- Config: `~/.vibe-action/config.yaml` — override with `VIBE_CONFIG`.
- Actions: `~/.vibe-action/actions/` — override with `VIBE_ACTION_PATH`.
- Both directories are created on first run. Custom `.yaml` files in the
  actions directory are loaded automatically.

## Initialization order

1. Output registry from `VIBE_LOG_TYPE` / `VIBE_TRACE_LEVEL`.
2. Config file from `VIBE_CONFIG` or default path; create if missing.
3. Validate config (version, cluster nodes).
4. Load pipelines (scan actions dir, validate, cache).
