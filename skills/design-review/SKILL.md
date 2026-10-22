---
name: design-review
description: Senior-style, prioritized design critique of a Rust desktop UI (Freya, egui, gpui) from a screenshot, a running window or component source. Covers visual hierarchy, typography, spacing, color and contrast, motion, component states, window resizing, accessibility, copy and consistency. Returns findings ranked Blocking, Important, Polish, each with a specific fix in the project's framework idiom, and can apply the safe ones with --apply. Use when the user asks for a design review, crit, craft check or "does this look right" before a review, handoff or release. Adapted for local Rust code from Superfuture's design-review skill. Usually invoked by the native-ux router; can be called directly.
---

# design-review

> **Stack:** Rust native crates load the `rust-gui-map` skill for every API name. Web and mobile projects use `native-ux/references/web-map.md` and the original web idioms. The rules below are stack-agnostic; only the fixes change idiom. Severity maps to the unified scale in the `native-ux` orchestrator when run through it.

Adapted from Joey Primiani's Superfuture `design-review`. The rubric is his. The telemetry ping and the license-gated Pro tier were removed; everything runs locally against Rust source and a native window.

A senior-level craft critique. Opinionated and specific, not a checklist dump.

## 1. Acquire the artifact

Resolve the stack first: Rust crates load the `rust-gui-map` skill; web and mobile use `native-ux/references/web-map.md`.

- **Screenshot or image**: read it. Best for visual judgment (hierarchy, type, spacing, color). Ask for the window size and theme (light or dark) if not obvious.
- **Component or screen source** (`.rs`): read it plus the theme or token module it pulls from. Enables implementation-level fixes with `file:line` references and `--apply`.
- **Running app**: use the `run` skill to launch and screenshot it at the default and minimum window sizes. For Freya, `freya-testing` `render_to_file` gives a deterministic render; for egui, `egui_kittest` snapshots.
- If you have neither a render nor code for a visual question, ask for one before guessing.

Index the crate with `codebase-memory-mcp` when the review spans more than one file so you can see how far a shared fix reaches.

## 2. Evaluate against the rubric

Review every dimension in `checklist.md` (same directory):
hierarchy and layout · typography · spacing and rhythm · color and contrast · motion · component states · accessibility · window sizes · content and copy · consistency and theme.

Check actual numbers where possible: contrast ratios from the theme's color values, line heights, measure in characters, hit target px, animation durations in `.time(..)`, `animate_bool_with_time`, `Duration::from_millis`. Numbers beat vibes.

## 3. Report: ranked, concrete, scannable

Group findings by severity, most important first. Lead with the few that matter; do not list everything. For each finding:

- **What**: the specific issue, quoting the element or `file.rs:line` when code is available.
- **Why**: the craft or usability reason, one line.
- **Fix**: a concrete change in the project's framework and its tokens. Exact value, not "increase spacing". `.spacing(16.)` not "more gap".

Severity tiers:

- 🔴 **Blocking**: broken, keyboard-unreachable, fails WCAG AA contrast, clipped at the minimum window size, or destructive with no confirmation.
- 🟠 **Important**: noticeably hurts hierarchy, readability, usability or consistency.
- 🟡 **Polish**: refinement that sharpens the craft.

End with **Strengths** (2 to 4 things done well) and, if useful, the single highest-leverage change to make first.

## 4. Optional: apply fixes

With `--apply` (or when the user asks), edit the code for the clear, safe findings: spacing and radius values, contrast pairs via the theme, focus styles, accessible names (`a11y_alt`, `widget_info`), truncation with tooltips, hit-area minimums. Leave subjective or structural changes as recommendations unless confirmed. After editing run `cargo check` (and `cargo clippy` if the project uses it) and re-verify the values you changed. Follow the `rust-engineering` skill for any Rust you write.

## Tone

Direct and respectful, like reviewing a colleague's work: precise about problems, never vague, always paired with the fix. Calibrate depth to the surface. A launcher or onboarding screen earns motion and type scrutiny; a settings form earns states and accessibility scrutiny; a dense tool panel earns density and keyboard scrutiny.
