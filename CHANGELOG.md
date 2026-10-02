# Changelog

All notable changes to Vibe Action will be documented in this file.

## [0.4.0] - 2026-10-02

### 🐛 Fixes

- Store VIBE_TRACE_LEVEL in registry and gate Debug/Trace dispatch in all output modes

### 📚 Documentation

- Add detailed verbosity section and update table

### 🚀 Features

- Add off guard for conditional action skipping

## [0.3.2] - 2026-09-30

### 🐛 Fixes

- Emit trace logs only in tracing mode and harden confirms for non-TTY CI runs

## [0.3.1] - 2026-09-29

### 📚 Documentation

- Update embedded docs action and module docs for action groups

### 🚀 Features

- Add action groups and external repositories
- Add code and shell language system providers with centralized extension mapping
- Add ci fixtures and fix fixture layout for nested groups
- Add groups export to JSON output

## [0.3.0] - 2026-09-25

### 🚀 Features

- Add strip modifier to remove common wrapper/fence delimiters and update documentation
- Upgrade pipeline and engine

## [0.2.3] - 2026-08-21

### 📚 Documentation

- Add empty stub for cross-documentation topic

### 🚀 Features

- Show flow version in status and add source path for error reporting
- Add json-mode confirm via stdin with fail-closed timeout and confirm output export

## [0.2.2] - 2026-08-18

### ⚡ Refactoring

- Add and use cache_version constant
- Simplify git commit action by removing echoed message

### 🐛 Fixes

- Prompt before output in non-json modes

### 📚 Documentation

- Rewrite user guides and consolidate book css
- Clarify guard condition for dry-run commit execution
- Document v0.2.2 release changes

### 🚀 Features

- Improve commit message synthesis and confirm completion for non-json outputs
- Print commit message before running git commit

## [0.2.1] - 2026-08-17

### 🚀 Features

- Add version fields and cache versioning

## [0.2.0] - 2026-08-16

### ⚡ Refactoring

- Switch default actions to dialog output
- Replace arboard with clipboard-rs

### 🐛 Fixes

- Remove is_prompt from custom help template rendering

### 📚 Documentation

- Add context blurb about vibe action and chat
- Add v0.2.0 changelog entry

### 🚀 Features

- Add clipboard screenshot support
- Add unified query tag with type modifiers for flexible input handling in pipelines
- Support multi-input actions and flexible output targets

## [0.1.5] - 2026-08-08

### 🚀 Features

- Conditional step execution, file inputs, and dry-run

## [0.1.4] - 2026-08-07

### 🐛 Fixes

- Enable zune-jpeg log feature to fix cargo install (0.1.4)

## [0.1.3] - 2026-08-07

### 🚀 Features

- Add file input and enhance output formatting

## [0.1.2] - 2026-08-01

### ⚡ Refactoring

- Improve process management and error handling
- Update message formatting
- Update markdown formatting

### 📚 Documentation

- Update documentation for v0.1.2 release

### 🚀 Features

- Add test output mode and refactor output system
- Add vibe_skip_lock and active run support
- Add singleton guard, update deps, fix progress
- Add run_guard module and update path
- Add conditional clipboard, markdown stripping, output export
- Implement api config and styled placeholders
- Add cli output check, api target, ide config
- Add stop and upgrade status
- Exit with code 130 when superseded, retry startup lock, and run clean under the run guard
- Improve cli status, help output, and trace validation
- Add ide plugin api integration across action configs

## [0.1.1] - 2026-07-04

### 🐛 Fixes

- Bump vibe-ast version and use head diff
- Disable context cache cleaning message

### 📚 Documentation

- Add system commands, benchmarks, and version bump

### 🚀 Features

- Add cli subcommands, modules, config version, and custom help
- Add clean command and refactor flow loading
- Add benchmark command and update dependencies
- Add temp file cleanup
- Add file locking and error handling for flow loading

## [0.1.0] - 2026-06-30

### ⚡ Refactoring

- Update code structure and documentation
- Introduce ArgActionModel, ArgActionValue, ValidateTrait, clean up CLI
- Update engine module by removing unused resolver and adding new action-related functionality
- Update and enhance various features and methods across the codebase.
- Update dependencies, models, and validation logic in multiple files.
- Improved CLI handling, performance, and error management across multiple functions.
- Improved CLI handling, performance, and readability across multiple files.
- Improve commit handling, code performance, and readability; fix bugs; update configurations.
- Update commit action, fill method, engine logic, and regex for enhanced functionality.
- Improve commit handling and action model.
- Optimize regex handling in exec_action
- Improve error handling and remove unused JSON variants.
- Improve comments in ExpandedTemplate struct
- Update various components for enhanced functionality and consistency.
- Improve execute function, update ClusterConfig, fix syntax error, add modules, refactor Engine, ActionModel, FlowModel.
- Simplify cluster creation and improve exec handling.
- Update actions and clean code
- Update dependencies, enhance README, and clean up code.
- Improve engine initialization, remove unused code, fix bugs
- Update fill method and add ModifierRegistry
- Improve AppConfig, CLI, and path handling.
- Clean up and enhance functionality
- Streamline action flows using structs and YAML templates
- Update flow trait with YAML
- Update YAML configurations for improved readability and functionality.
- Improve action handling and conditions
- Update logging and AppConfig structure.
- Update LLM commands and rename tasks
- Consolidate time formatting and utilities
- Add width parameter to render_markdown
- Rename models and add vision support
- Whois action to analyze multiple tech figures
- Update FAQ and fix double-brace escapes
- Change expect to input for arguments
- Remove caching mechanism

### 🐛 Fixes

- Up fixture
- Remove extra newline in confirmation prompt
- Update search query in find benchmark

### 📚 Documentation

- Fix typos and improve grammar in README.md
- Add documentation URL to Cargo.toml
- Improve README clarity and consistency
- Improve README and add new features.
- Add new section on IDE integration
- Update documentation and commit action
- Add changelog config and update changelog

### 🚀 Features

- Initial commit — AI-native command router
- Actions model, defaults, save/load with YAML headers
- Dynamic CLI from YAML actions, args support, action dispatch
- Enhance file finder, refactor error handling, add tracking for processed tags.
- Add dynamic log level configuration and enhance command routing
- Add regex matching, new modifiers, and update dependencies.
- Add switch
- Add notify-rust dependency and update documentation
- Update documentation
- Add new tasks, dependencies, and refactor LLM variants.
- Add new actions and update translations
- Add role mismatch check and confirm prompt
- Add Markdown rendering with syntax highlighting
- Bump version, add describe, whois, and benchmarks
- Add FAQ section and refactor engine
- Add vision support and built-in actions
- Add http client, fetch, and image handling
- Add pdf fetching with deterministic caching
- Add sysinfo command and system tags support
- Add hostname, arch, shell to sysinfo
- Add fetch action and update documentation
- Add format and scan modifiers, new actions
- Add project-export action and various enhancements
- Add clipboard support to multiple actions
- Add clipboard modifier and rename project-export to scan

