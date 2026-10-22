# Playbooks

Each playbook is a sequence. Read-only steps never touch source. Steps marked **confirm** need the user's go-ahead before edits. Every playbook ends by merging into the report skeleton in `SKILL.md`.

## P1. New native app from zero (Rust)

Trigger: "quiero hacer una app de escritorio", "nuevo proyecto freya/egui/gpui", "arma la UI de…".

1. `rust-gui-map`: detect or choose the framework; state the idioms the build will use.
2. `map-user-flows` in **plan mode**: with no code yet, derive screens, phases and stories from the description. Output becomes the build plan. Ask the entry point and platform if ambiguous.
3. `laws-of-ux` build rules: one primary action per view, chunking, defaults, states, targets, progress. Decide before coding.
4. Build with `rust-engineering` (and `freya` when applicable). Theme tokens first, components second, screens third. Every screen gets loading, empty and error branches from day one.
5. `spacing-audit` on the new code, summary only, then snap.
6. `better-interface` quick mode on the entry flow.
7. `find-animation-opportunities` in reject-heavy posture; implement at most the top one if the user wants motion.
8. Report: the built artifact, a "UX decisions" note, the quick review findings.

## P2. Company website or landing page (web)

Trigger: "quiero hacer una web para mi empresa", "landing", "sitio".

1. Stack is web. Load `references/web-map.md`.
2. `laws-of-ux` build rules plus `areas/navigation-choices.md` and `areas/layout-content.md`. Decide: one dominant CTA per view, conventional nav (Jakob), scannable sections (chunking, serial position), real content hierarchy.
3. If an aesthetics skill is installed (`impeccable`, `frontend-design`, `design-taste-frontend`, `high-end-visual-design`, `gpt-taste`), run it for visual direction. This plugin does not pick fonts or palettes.
4. Build. Semantic HTML, visible focus, labels, `prefers-reduced-motion`, responsive from 320px.
5. `design-review` on the result with `laws-of-ux` areas loaded. Blocking items fixed before delivery.
6. `find-animation-opportunities`: marketing pages are where the delight budget lives, but hero and nav still obey the gate.
7. Report: artifact, UX decisions note, review findings.

## P3. Pricing page

Trigger: "página de precios", "planes", "pricing".

1. `laws-of-ux` `areas/navigation-choices.md` pricing block: 3 to 4 self-serve tiers in the row, enterprise separated, one recommended plan, differentiators only, comparison table below.
2. Build in the stack's idiom.
3. `design-review`: Von Restorff on the recommended plan, Similarity across cards, hit areas on toggles.
4. `laws-of-ux` ethics: no fake scarcity, no confirmshaming on downgrade.

## P4. Redesign an existing screen or app

Trigger: "rediseñar", "modernizar", "se ve viejo", "hazlo premium", "mejora la UI".

1. Detect stack. Rust: `rust-gui-map` and index with `codebase-memory-mcp`.
2. `analyze-components` read-only: inventory, duplicates, hardcoded tokens. The redesign must not multiply components.
3. `design-review` on the current state as the **baseline**. Save it; it is the before.
4. `map-user-flows` if the redesign touches more than one screen, to catch missing states before restyling.
5. Plan: list the findings the redesign will resolve, the tokens it will introduce, the components it will consolidate. **confirm**.
6. Implement with `rust-engineering` or the web aesthetics skill. Tokens first.
7. `spacing-audit` summary then snap.
8. `better-interface` full on the result. Compare against the baseline; every baseline Blocking must be gone.
9. `find-animation-opportunities` only if the user asked for "feel"; otherwise skip.
10. Report with a before/after table of resolved findings.

## P5. Pre-ship audit

Trigger: "antes de lanzar", "is it ready", "auditoría", "release".

1. Stack detection and index.
2. `better-interface` full, all five domains, with `laws-of-ux` areas `feedback-states`, `forms-input`, `ethics` loaded for Why cells.
3. `spacing-audit` summary only (no edits); its systemic finding becomes one row.
4. `design-review` contrast numbers (§4) if the theme has not been measured.
5. Verdict `Block` or `Approve`. Blocking list first. Offer `--apply` for the safe ones.

## P6. Change review (branch, PR, diff)

Trigger: a branch name, PR number, "lo que cambié", "revisa mi diff".

1. Scope = the diff only. Index or `detect_changes`.
2. `better-interface` change-scoped: findings tagged Introduced / Regression / Pre-existing.
3. `spacing-audit` on changed files only.
4. If the diff adds animation: `find-animation-opportunities` gate on each new animation (frequency first).
5. Verdict covers Introduced and Regression only.

## P7. Motion pass

Trigger: "animar", "más vivo", "se siente muerto", "feel".

1. `rust-gui-map` motion column, or `web-map.md`.
2. `find-animation-opportunities` read-only. Rejected list is mandatory.
3. If the user wants implementation: take the top 1 to 3 rows, implement with `rust-engineering`, respect an existing reduce-motion setting or add one once.
4. `better-interface` polish domain on the implemented motion (interruptible, no first-render fire, right properties).

## P8. Spacing cleanup

Trigger: "espaciado", "desalineado", "4/8", "ritmo".

1. `rust-gui-map` spacing row.
2. `spacing-audit` summary. **confirm**. Snap. `cargo check`.
3. If the audit finds no scale module, propose one token module as a single Important finding; do not create it unasked.

## P9. Component cleanup / design system

Trigger: "limpiar componentes", "duplicados", "design system", "inventario".

1. Index with `codebase-memory-mcp`.
2. `analyze-components`: writes `docs/component-audit.md`. Read-only.
3. **confirm** the fix plan. Apply in order: delete orphans, consolidate duplicates, extract tokens, add missing states, rename.
4. `better-interface` quick on the consolidated components.

## P10. PRD, user stories, flow map from code

Trigger: "user stories", "mapa de flujos", "PRD", "qué pantallas faltan", "handoff".

1. Index. `rust-gui-map` navigation row to find routes.
2. `map-user-flows`: writes `docs/user-flows.md` with Mermaid and action items. Read-only.
3. If the user then wants the gaps built, each action item carries a paste-ready prompt; run P1 steps 3 to 6 per gap.

## P11. Onboarding

Trigger: "onboarding", "primer uso", "tutorial", "registro".

1. `laws-of-ux` `areas/onboarding-flows.md`: value before effort, defer fields, progress, head start, finish strong.
2. `map-user-flows` on the onboarding screens to catch dead ends.
3. Build or review per intent; `better-interface` writing domain on the copy.

## P12. Forms and checkout

Trigger: "formulario", "checkout", "pago", "validación", "abandono".

1. `laws-of-ux` `areas/forms-input.md` including the code red flags for the stack.
2. `better-interface` accessibility and writing domains (labels, errors, keyboard).
3. `design-review` §6 states: loading on submit, never wipe data on error, one dominant submit.
4. `laws-of-ux` ethics on defaults and consent.

## P13. Dashboard or data-dense tool

Trigger: "dashboard", "tabla", "métricas", "editor", "panel".

1. `laws-of-ux` Pareto and cognitive load: surface the few metrics, the rest on demand.
2. `better-interface` with density allowance: compact spacing is fine when hit areas do not overlap.
3. `find-animation-opportunities` in reject mode: data the user reads never animates for style.
4. Typography: tabular numbers, truncation with full value reachable.

## P14. Accessibility pass

Trigger: "accesibilidad", "teclado", "screen reader", "contraste", "a11y".

1. `rust-gui-map` keyboard/focus/a11y rows (what the toolkit can and cannot expose).
2. `better-interface` accessibility domain only, every trigger is Blocking.
3. `design-review` §4 contrast numbers for both theme variants.
4. Where the toolkit cannot expose a name or role (gpui today), one Important finding naming the platform gap, not one per element.

## P15. Theme, dark mode, tokens

Trigger: "tema", "dark mode", "modo oscuro", "tokens", "colores".

1. `analyze-components` hardcoded-style detection only.
2. `design-review` §4 and §10: contrast in both modes, consistent radii and elevation.
3. `better-interface` polish: image outlines per mode, icon recoloring from `currentColor` equivalents.

## P16. Internationalization

Trigger: "traducir", "i18n", "idiomas", "RTL".

1. `better-interface` writing domain: no concatenated fragments, pluralization, one voice.
2. `better-interface` layout: growth and clipping with long strings; leading/trailing not left/right.
3. For Freya: `freya-i18n` and `.ftl`; the review reads the strings there.

## P17. Quick question

Trigger: one specific doubt, a threshold, "which law applies".

1. `laws-of-ux` quick mode only. Under 200 words, one law at most, answer then reason then fix.
2. No other skill runs. No report skeleton.

## P18. Visual-only review (screenshot, Figma, Penpot)

Trigger: an image or design link and "qué opinas", "revisa".

1. `design-review` from the render. Ask the viewport and theme.
2. `laws-of-ux` areas matching the surface for Why.
3. Code-level domains (keyboard, names, states not visible) marked Not verified. Offer to review the source.

## Combining rules

- One full review per scope per turn.
- Inventory and flows before restyling. Never redesign what you have not mapped.
- Spacing and motion after structure. Never polish a layout that will change.
- Build intents always end with a review loop on the artifact, quick depth unless the user asked for full.
- Every edit step is preceded by the summary it is based on and, where marked, by confirmation.
