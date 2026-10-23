# definitive-ux

The definitive UI/UX skill set for coding agents. One orchestrator (`definitive-ux`) classifies any design, build, redesign, review, polish, motion, spacing, component, token or flow request, detects the stack and the form factor, and runs the right specialist skills in the right order with one consolidated report.

Native first, generic by design. Verified maps for Rust: **Freya 0.5**, **egui 0.32**, **gpui 0.2**, **Slint 1.x**, **Leptos 0.8**; maps for Iced, Dioxus, ratatui, web, SwiftUI, Compose, Flutter and React Native; platform conventions for desktop, mobile, TV, watch, embedded, web and terminal UIs. One design source (DESIGN.md + tokens) derived into every target so the same product feels native everywhere.

## Install

`definitive-ux` is a standard [Agent Skills](https://agentskills.io) repository, so it works in any host that reads `SKILL.md` files.

### Claude Code (plugin, includes the auto-trigger hook)

```bash
claude plugin marketplace add SergioRibera/definitive-ux
claude plugin install definitive-ux@definitive-ux
```

### Codex, Cursor, Gemini CLI, OpenCode, GitHub Copilot, Amp, Windsurf, Cline, Roo and others

```bash
npx skills add SergioRibera/definitive-ux            # pick agents interactively
npx skills add SergioRibera/definitive-ux -a "*" -g  # every detected agent, user-level
npx skills add SergioRibera/definitive-ux -a codex -a cursor -a opencode -y
```

The `skills` CLI symlinks each skill into the right place per host (`~/.codex/skills`, `.cursor/skills`, `.opencode/skill`, `.github/skills`, `.agents/skills`, ...). Each skill ships `agents/openai.yaml` so Codex shows a display name and a default prompt.

### Manual

Copy `skills/*` into your host's skills directory. Skill names are bare (`definitive-ux`, `design-review`, ...). Claude Code namespaces them under the plugin (`/definitive-ux:design-review`).

### Auto-triggering outside Claude Code

Only Claude Code runs the `UserPromptSubmit` hook. On every other host, paste `templates/AGENTS.snippet.md` into your project's `AGENTS.md`, `GEMINI.md` or rules file so the agent loads `definitive-ux` first for UI/UX work; the skill descriptions handle the rest.
## Skills

| Skill | Job | Source it adapts |
| --- | --- | --- |
| `definitive-ux` | Orchestrator: intent, stack, form factor, register, playbooks, one report | original |
| `stack-map` | Detects the framework and loads one verified API map (Freya, egui, gpui, Slint, Iced, Dioxus, Leptos, ratatui, web, SwiftUI, Compose, Flutter, React Native) | original |
| `laws-of-ux` | The 30 Laws of UX as decisions: quick answers, audit and build rules, thresholds, ethics | lawsofux.com (Jon Yablonski) |
| `design-review` | Prioritized craft critique with computed contrast, visual passes and a 0–4 score | Superfuture design-review, taste-skill critique, impeccable audit |
| `better-interface` | Five-domain system review with Block / Approve verdict | Jakub Krehel |
| `design-source` | DESIGN.md and three-layer tokens derived into every target theme | ui-ux-pro-max tokens, Anti-Slop and impeccable DESIGN.md |
| `art-direction` | Decided, non-generic direction for persuade surfaces, look-and-fix passes | taste-skill, impeccable brand register |
| `find-animation-opportunities` | Where motion is missing and where it must never be | Emil Kowalski |
| `animate` | Build one animation that survives review | Emil Kowalski |
| `improve-animations` | Audit existing motion, write executable plans | Emil Kowalski |
| `spacing-audit` | 4/8pt audit of padding, gaps and margins | Figma community |
| `analyze-components` | Component inventory via the code graph: orphans, duplicates, drift, literals | Amy Ha |
| `map-user-flows` | Screens to user stories, Mermaid flows, action items | Amy H. |
| `break-ui` | Realistic worst-case data behind a toggle | Emil Kowalski |

Shared references in `skills/definitive-ux/references/`: `playbooks.md` (24 use cases), `platforms/` (desktop, mobile, tv, watch, embedded, web, tui), `motion-standards.md`, `slop-tells.md`.

Invoke the orchestrator with `/definitive-ux:definitive-ux`, or any specialist directly, for example `/definitive-ux:design-review`.
## How routing works

The orchestrator never reviews anything itself. It resolves intent and stack, then names the skills to run, their order, and the output contract. `skills/definitive-ux/references/playbooks.md` holds the strategies for every use case (new app, company website, redesign, pre-ship audit, PR review, motion pass, component cleanup, PRD from code, quick question, dark-pattern check, and more).

## Evals

`evals/` follows the `claude plugin eval` layout (one directory per case: `case.yaml`, `prompt.md`, `graders/`, optional `files/`). Cases cover React, description-only, Rust (Freya, egui) and a multi-target design-source workspace (Freya desktop, Slint HMI, Leptos web).

## License

MIT for this repository. Adapted skills credit their original authors inside each `SKILL.md`; the Laws of UX summaries are written in our own words and link to lawsofux.com for the full text.
