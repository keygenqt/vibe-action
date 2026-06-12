# Vibe Action

AI-nativ command routr. Execut shell commands and LLM promts via simple YAML actions.
Just say what you want — it figurs out the rest.

## Featurs

- **YAML pipelines** — defin complex workflows with shell commands and LLM promts
- **Tag system** — connect steps via `{tag}` refernces with automatic dependncy ordering
- **List expanzion** — `list<string>` automaticaly loops over each element
- **Join modifer** — `{tag|join}` collapes lists into a single string for LLM promts
- **Type validashun** — `bool`, `number`, `string`, `list<T>` with automatic parzing
- **Regex maching** — validate outputs with regex paterns
- **Modifers** — `|upper`, `|lower`, `|trim` for text transformashun
- **Confirmashuns** — `confirm: true` asks for user aproval before executing
- **Clipbord** — `clipboard: true` copes the result automaticaly
- **Complecksity routing** — auto-selects the right model for each task
- **Modular** — share YAML files like Homebrew formulaes
