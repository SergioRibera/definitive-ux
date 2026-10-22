# native-ux

The definitive native UI/UX skill set for Claude Code. One orchestrator (`native-ux`) classifies any design, build, redesign, review, motion, spacing, component or flow request, detects the stack, and runs the right specialist skills in the right order with one consolidated report.

First-class target: Rust desktop apps built with **Freya 0.5**, **egui 0.32** and **gpui 0.2**. Web and mobile are supported through the same principles with their own idioms.

## Install

`native-ux` is a standard [Agent Skills](https://agentskills.io) repository, so it works in any host that reads `SKILL.md` files.

### Claude Code (plugin, includes the auto-trigger hook)

```bash
claude plugin marketplace add SergioRibera/native-ux
claude plugin install native-ux@native-ux
```

### Codex, Cursor, Gemini CLI, OpenCode, GitHub Copilot, Amp, Windsurf, Cline, Roo and others

```bash
npx skills add SergioRibera/native-ux            # pick agents interactively
npx skills add SergioRibera/native-ux -a "*" -g  # every detected agent, user-level
npx skills add SergioRibera/native-ux -a codex -a cursor -a opencode -y
```

The `skills` CLI symlinks each skill into the right place per host (`~/.codex/skills`, `.cursor/skills`, `.opencode/skill`, `.github/skills`, `.agents/skills`, ...). Each skill ships `agents/openai.yaml` so Codex shows a display name and a default prompt.

### Manual

Copy `skills/*` into your host's skills directory. Skill names are bare (`native-ux`, `design-review`, ...). Claude Code namespaces them under the plugin (`/native-ux:design-review`).

### Auto-triggering outside Claude Code

Only Claude Code runs the `UserPromptSubmit` hook. On every other host, paste `templates/AGENTS.snippet.md` into your project's `AGENTS.md`, `GEMINI.md` or rules file so the agent loads `native-ux` first for UI/UX work; the skill descriptions handle the rest.
## Skills

| Skill | Job | Source it adapts |
| --- | --- | --- |
| `native-ux` | Orchestrator: intent router, stack detection, playbooks, one report | original |
| `rust-gui-map` | Verified API map for Freya / egui / gpui: where spacing, focus, a11y, motion and routes live | original |
| `laws-of-ux` | The 30 Laws of UX as decisions: quick answers, audit rules, build rules, thresholds, ethics | lawsofux.com (Jon Yablonski) |
| `design-review` | Prioritized craft critique: Blocking / Important / Polish, `--apply` | Superfuture design-review |
| `better-interface` | Five-domain system review with Block / Approve verdict | Jakub Krehel better-interface |
| `find-animation-opportunities` | Where motion is missing and where it must never be | Emil Kowalski |
| `spacing-audit` | 4/8pt audit of padding, gaps and margins in code | Figma community |
| `analyze-components` | Component inventory via the code graph: orphans, duplicates, drift | Amy Ha |
| `map-user-flows` | Screens to user stories, Mermaid flows and action items | Amy H. |

Invoke the orchestrator with `/native-ux:native-ux`, or any specialist directly, for example `/native-ux:design-review`.

## How routing works

The orchestrator never reviews anything itself. It resolves intent and stack, then names the skills to run, their order, and the output contract. `skills/native-ux/references/playbooks.md` holds the strategies for every use case (new app, company website, redesign, pre-ship audit, PR review, motion pass, component cleanup, PRD from code, quick question, dark-pattern check, and more).

## Evals

`evals/` follows the `claude plugin eval` layout (one directory per case: `case.yaml`, `prompt.md`, `graders/`, optional `files/`). Cases cover React, description-only and Rust (Freya, egui) fixtures.

## License

MIT for this repository. Adapted skills credit their original authors inside each `SKILL.md`; the Laws of UX summaries are written in our own words and link to lawsofux.com for the full text.
