<!-- Paste into your project's AGENTS.md / CLAUDE.md / GEMINI.md / .cursor rules.
     Replaces the Claude Code hook on hosts that have no prompt hooks. -->

## UI/UX work

For anything about an interface, screen, app, website, landing, pricing page, dashboard, form, onboarding, component, layout, theme, icons, typography, spacing, animation, usability, accessibility or user flow, in Rust desktop apps (Freya, egui, gpui), web or mobile: load the `native-ux` skill FIRST and follow its router. Classify the intent (question, build, redesign, audit, motion, spacing, components, flows), detect the stack (`rust-gui-map` for Rust crates, `native-ux/references/web-map.md` for web and mobile), then run only the specialist skills the router names (`design-review`, `better-interface`, `laws-of-ux`, `find-animation-opportunities`, `spacing-audit`, `analyze-components`, `map-user-flows`) in the order it gives. Produce one consolidated report with the unified severity scale, not one report per skill. Review requests are read-only unless the user asks for edits.
