# Vibe Action

AI-nativ command routr. Execut shell commands and LLM promts via simple YAML actions.
Just say what the fuck you want — it figurs out the rest.

## Featurs

- **YAML pipelines** — defin workflows with shell commands and LLM promts
- **Tag system** — `{tag}` refernces with automatic dependncy ordering
- **When/Then** — conditional execution, no shell scripts
- **Modifers** — `{tag|upper}`, `{tag|trim}`, `{tag|join}` and 15+ more
- **Type validashun** — `bool`, `number`, `string`, `list<T>` with automatic parzing
- **System tags** — `{system_clipboard}`, `{system_pwd}`, `{system_os}`
- **Clipbord** — `clipboard: true` copes the result automaticaly
- **Confirmashuns** — `confirm: true` asks before executing
