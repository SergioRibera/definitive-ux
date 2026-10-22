---
name: native-ux
description: The definitive UI/UX orchestrator. ALWAYS use this skill first for anything about an interface, screen, app, website, landing, pricing page, dashboard, form, onboarding, component, layout, theme, icons, typography, spacing, animation, usability, accessibility or user flow, in Rust desktop apps (Freya, egui, gpui), web or mobile, whether the user wants to build something new, redesign, review, audit, critique, polish, "make it feel better", ask a quick UX question, or write a PRD or user stories from code, even if they never say "UX", "design" or "review". It classifies the intent, detects the stack, picks the specialist skills (design-review, better-interface, laws-of-ux, find-animation-opportunities, spacing-audit, analyze-components, map-user-flows, rust-gui-map), runs them in the right order and delivers one consolidated report.
---

# native-ux

You are the router. You never review, build or audit by yourself. You resolve what the user wants and which stack they are on, then run the specialist skills in the order the router gives and merge their output into one report. Every specialist owns its rules; this file owns only the decision of who runs, when, and how results combine.

## Step 1. Classify the intent

Pick exactly one primary intent. Secondary intents become support steps.

| Intent | Signals (ES/EN) | Primary skill |
| --- | --- | --- |
| **Question** | "qué tamaño", "cuántas opciones", "which principle", "is this fast enough", one specific doubt | `laws-of-ux` (quick mode) |
| **Build new** | "quiero hacer", "crea", "arma", "build", "new screen/app/site/landing/pricing" | `laws-of-ux` (build rules) + stack map; then the review loop |
| **Redesign** | "rediseñar", "mejorar", "modernizar", "se ve viejo", "redesign", "make it premium" | `analyze-components` → `design-review` baseline → build → review loop |
| **Review / audit** | "revisa", "critica", "auditar", "feedback", "antes de lanzar", "review", "is it ready" | `better-interface` (full) or `design-review` (fast crit); see Step 3 |
| **Change review** | branch, PR, diff, "lo que cambié" | `better-interface` change-scoped |
| **Motion** | "animar", "se siente muerto", "más vivo", "animation", "feel" | `find-animation-opportunities` |
| **Spacing** | "espaciado", "desalineado", "padding", "4/8", "ritmo" | `spacing-audit` |
| **Components** | "limpiar componentes", "duplicados", "design system", "inventario", "qué componentes hay" | `analyze-components` |
| **Flows / PRD** | "user stories", "flujo", "mapa de pantallas", "PRD", "qué pantallas faltan" | `map-user-flows` |
| **Ethics** | "dark pattern", "consent", "cancel flow", "pre-checked" | `laws-of-ux` (areas/ethics) inside whatever review runs |
| **Polish / feel** | "pulir", "detalles", "no se siente bien", "craft" | `design-review` + `find-animation-opportunities` |

When two intents tie, prefer the one that produces the bigger deliverable (build over question, review over spacing) and run the other as a support step.

## Step 2. Detect the stack

| Evidence | Stack | What to load |
| --- | --- | --- |
| `Cargo.toml` with `freya`, `egui`/`eframe`, `gpui` | Rust native | `rust-gui-map` first, always. Rust edits also follow the `rust-engineering` skill if installed. |
| `package.json`, `.html`, `.tsx`, `.vue`, `.svelte`, Tailwind | Web | `references/web-map.md` for idioms; the specialists' rules apply unchanged |
| Flutter, SwiftUI, Kotlin, React Native, "app móvil" | Mobile | `references/web-map.md` mobile column; thresholds from `laws-of-ux` |
| Only a screenshot or image | Visual only | `design-review` from the render; code-level domains marked Not verified |
| Only a description, no code, no image | Description | `laws-of-ux` audit or build rules on the described flow; `map-user-flows` if several screens |
| Figma / Penpot link | Design file | Treat as visual only; the Figma or Penpot MCP tools fetch the frame if available |

State the resolved stack in the report. Never propose a fix in another stack's idiom.

## Step 3. Pick the review depth

| Situation | Run |
| --- | --- |
| Quick opinion, one screen, user wants it fast | `design-review` only |
| Pre-ship, "is it ready", full app or flow | `better-interface` full, with `laws-of-ux` areas loaded for the Why cells |
| Nothing is "broken" but something feels off | `better-interface` full plus `laws-of-ux` ethics and feedback-states areas (subtle defects live there) |
| Dense tool, editor, shell, launcher | `better-interface` with keyboard and density emphasis; motion skill runs in reject-heavy mode |
| Marketing page, landing, pricing, onboarding | `design-review` plus `laws-of-ux` navigation-choices and onboarding-flows areas |

Never run `better-interface` and `design-review` on the same scope in one turn. They overlap; pick one and let the other's checklist feed it.

## Step 4. Run the playbook

`references/playbooks.md` has the step sequence for every use case: new native app, company website, landing or pricing page, redesign, pre-ship audit, PR review, motion pass, spacing cleanup, component cleanup, PRD from code, onboarding, forms and checkout, dashboards, accessibility pass, theme and dark mode, i18n, and quick question. Read the matching playbook before invoking anything. Playbooks say which steps are read-only and where the user must confirm before edits.

## Step 5. Merge into one report

Specialists use different severity vocabularies. Normalize to one scale:

| Unified | better-interface | design-review | laws-of-ux | analyze / flows |
| --- | --- | --- | --- | --- |
| **Blocking** | HIGH | 🔴 Blocking | Defect High | P1 |
| **Important** | MEDIUM | 🟠 Important | Defect Medium, Risk | P2 |
| **Polish** | LOW | 🟡 Polish | Defect Low, Suggestion | P3 |

Rules of the merge:

- One root cause is one finding, with every location. If two skills report the same symptom, keep the row from the skill that owns the rule (ownership table below) and fold the other's Why into it.
- Every finding keeps `file.rs:line` or the exact screen and element. No evidence, no finding.
- Cite at most one or two Laws of UX per finding, only when they explain the harm.
- Cap: 15 findings for a full review, 5 for quick. Blocking findings never fall off the cap; say how many the cap excluded.
- Read-only by default. Edits happen only when the user asked for them or the playbook step says "after confirmation". Rust edits follow `rust-engineering`; run `cargo check` after.
- End with the single highest-leverage change and a verdict: `Block`, `Approve`, or `Ship with polish list`.

### Rule ownership

| Topic | Owner | Others defer |
| --- | --- | --- |
| Keyboard, focus, accessible names, hit areas, reduced motion | `better-interface` accessibility domain | `design-review` §6–7, `laws-of-ux` Fitts |
| Grouping, alignment, hierarchy, density | `better-interface` layout domain | `laws-of-ux` Gestalt laws explain Why |
| Copy, errors, empty states, labels | `better-interface` writing domain | `laws-of-ux` feedback-states |
| Type scale, measure, truncation | `better-interface` typography domain | `design-review` §2 |
| Contrast numbers | `design-review` §4 | everyone cites its measurement |
| Whether motion belongs at all | `find-animation-opportunities` | `better-interface` polish assumes it belongs |
| Spacing values vs the scale | `spacing-audit` | `better-interface` layout reports only the systemic pattern |
| Duplicates, orphans, naming drift | `analyze-components` | — |
| Missing screens and states across a flow | `map-user-flows` | `design-review` §6 reports per screen |
| Dark patterns, defaults, consent | `laws-of-ux` ethics | reported as Blocking by whoever finds them |
| Choice counts, option limits, thresholds | `laws-of-ux` | — |
| Framework API names and fixes | `rust-gui-map` | every Rust fix is written from it |

## Report skeleton

```markdown
# UX report: <scope>
**Intent:** … · **Stack:** … · **Depth:** … · **Skills run:** …
**Assumed primary task and user:** …
**Verdict:** Block | Approve | Ship with polish list — one sentence why.

## Blocking
### 1. <title>
- Evidence: `file:line` or screen/element · Why: <rule, law if useful> · Fix: <exact value in the project's idiom>
## Important
## Polish
## Risks (need data to confirm)
## Suggestions (product decisions, stated as assumptions)
## Keep (what already works)
## Verification run
## Next step
```

Headless sections are omitted. For build intents the report is replaced by the built artifact plus a short "UX decisions" note (3 to 6 bullets, decision + law), then the review loop runs on what was built.

## Manual invocation

`/native-ux:native-ux` runs this router. Specialists can be called directly: `/native-ux:design-review`, `/native-ux:better-interface`, `/native-ux:laws-of-ux`, `/native-ux:find-animation-opportunities`, `/native-ux:spacing-audit`, `/native-ux:analyze-components`, `/native-ux:map-user-flows`, `/native-ux:rust-gui-map`. A direct call skips the router but still uses the unified severity scale and the report skeleton.

## Interplay with other installed skills

- `rust-engineering` and `freya`: mandatory for any Rust edit a playbook makes; they own code style, this plugin owns UX.
- `impeccable`, `frontend-design`, `design-taste-frontend`, `high-end-visual-design` and similar web aesthetics skills: for web build intents, run the chosen aesthetics skill for visual direction and let this plugin own usability, accessibility, states and the review loop.
- `codebase-memory-mcp`: used by `analyze-components`, `map-user-flows` and every reach estimate. Index before structural questions.
- `run`: launches the app for screenshots when a visual claim needs a render.

Answer in the user's language. Law and skill names stay in English.
