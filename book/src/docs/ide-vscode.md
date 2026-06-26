# VS Code Integration

Vibe Action integrates with VS Code through Tasks — no extension required.

## How it works

1. Select text in editor → `Cmd+C`
2. Run task → flow processes text from clipboard
3. System notification when done → `Cmd+V` to paste result

## Setup

### 1. Create Tasks

Create `.vscode/tasks.json` in your project:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Comment",
      "type": "shell",
      "command": "vibe-action comment",
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "focus": false
      }
    },
    {
      "label": "Explain",
      "type": "shell",
      "command": "vibe-action explain",
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "focus": false
      }
    },
    {
      "label": "Tone",
      "type": "shell",
      "command": "vibe-action tone",
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "focus": false
      }
    },
    {
      "label": "Spellcheck",
      "type": "shell",
      "command": "vibe-action spellcheck",
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "focus": false
      }
    },
    {
      "label": "Review",
      "type": "shell",
      "command": "vibe-action review",
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "focus": false
      }
    },
    {
      "label": "Translate Fast",
      "type": "shell",
      "command": "vibe-action translate-fast",
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "focus": false
      }
    },
    {
      "label": "Translate Deep",
      "type": "shell",
      "command": "vibe-action translate-deep",
      "problemMatcher": [],
      "presentation": {
        "reveal": "never",
        "focus": false
      }
    }
  ]
}
```

Key settings:

- `reveal: "never"` — don't show terminal
- `focus: false` — stay in editor

### 2. Install Task Notifier

Install [Task Notifier](https://marketplace.visualstudio.com/items?itemName=joelrobichaud.task-notifier) extension for system notifications on task completion.

### 3. Run Tasks

- `Cmd+Shift+P` → `Tasks: Run Task` → select command
- Or right-click in editor → `Tasks: Run Task`

### 4. Optional: Keyboard Shortcuts

Add to `keybindings.json`:

```json
[
  { "key": "f5", "command": "workbench.action.tasks.runTask", "args": "Comment" },
  { "key": "f6", "command": "workbench.action.tasks.runTask", "args": "Tone" },
  { "key": "f7", "command": "workbench.action.tasks.runTask", "args": "Spellcheck" },
  { "key": "f8", "command": "workbench.action.tasks.runTask", "args": "Review" }
]
```

## Notes

- Tasks execute in background — no terminal focus stealing
- Task Notifier provides system notifications (macOS, Linux, Windows)
- `.vscode/tasks.json` can be committed to the project
- Works with any Vibe Action flow, not just built-in ones
