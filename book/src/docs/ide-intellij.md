# IntelliJ IDEA Integration

Vibe Action integrates with IntelliJ IDEA through External Tools — no plugin required.

## How it works

1. Select text in editor → `Cmd+C`
2. Run External Tool → flow processes text from clipboard
3. Notification when done → `Cmd+V` to paste result

## Setup

### 1. Add External Tools

Via GUI:

`Preferences → Tools → External Tools → +`

| Field             | Value                         |
| ----------------- | ----------------------------- |
| Name              | `Vibe: Tone`                  |
| Group             | `External Tools`              |
| Description       | `Rewrite tone professionally` |
| Program           | `vibe-action`                 |
| Arguments         | `tone`                        |
| Working directory | `$ProjectFileDir$`            |

Repeat for other commands:

- `Vibe: Comment` — `comment`
- `Vibe: Spellcheck` — `spellcheck`
- `Vibe: Review` — `review`
- `Vibe: Explain` — `explain`
- `Vibe: Translate Fast` — `translate-fast`
- `Vibe: Translate Deep` — `translate-deep`

Or via XML — save to `~/Library/Application Support/JetBrains/<IDE>/tools/External Tools.xml` (macOS) or `~/.config/JetBrains/<IDE>/tools/External Tools.xml` (Linux):

```xml
<toolSet name="External Tools">
  <tool name="Vibe: Comment" description="Replace TODO with meaningful comment" showInMainMenu="false" showInEditor="false" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="vibe-action" />
      <option name="PARAMETERS" value="comment" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="Vibe: Explain" description="Add detailed comments to code" showInMainMenu="false" showInEditor="false" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="vibe-action" />
      <option name="PARAMETERS" value="explain" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="Vibe: Tone" description="Rewrite tone professionally" showInMainMenu="false" showInEditor="false" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="vibe-action" />
      <option name="PARAMETERS" value="tone" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="Vibe: Spellcheck" description="Fix spelling" showInMainMenu="false" showInEditor="false" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="vibe-action" />
      <option name="PARAMETERS" value="spellcheck" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="Vibe: Review" description="Critically analyze code" showInMainMenu="false" showInEditor="false" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="vibe-action" />
      <option name="PARAMETERS" value="review" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="Vibe: Translate Fast" description="Translate quickly" showInMainMenu="false" showInEditor="false" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="vibe-action" />
      <option name="PARAMETERS" value="translate-fast" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="Vibe: Translate Deep" description="Two-stage translation" showInMainMenu="false" showInEditor="false" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="vibe-action" />
      <option name="PARAMETERS" value="translate-deep" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
</toolSet>
```

Replace `<IDE>` with your version, e.g. `IntelliJIdea2026.1`.

### 2. Run Tools

- `Tools → External Tools → Vibe: Tone`
- Right-click in editor → `External Tools → Vibe: Tone`
- `Cmd+Shift+A` → type `Vibe: Tone`

### 3. Optional: Keyboard Shortcuts

`Preferences → Keymap` → search `Vibe: Tone` → assign shortcut.

## Notes

- External Tools run in background — no terminal popup
- IntelliJ shows built-in notification on tool completion
- XML config is global, works across all projects
- Works with any Vibe Action flow, not just built-in ones
