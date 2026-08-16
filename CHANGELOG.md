# Changelog

All notable changes to Vibe Action will be documented in this file.

## [0.2.0] - 2026-08-16

### ⚡ Refactoring

- Switch default actions to dialog output
- Replace arboard with clipboard-rs

### 🐛 Fixes

- Remove is_prompt from custom help template rendering

### 📚 Documentation

- Add context blurb about vibe action and chat

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

