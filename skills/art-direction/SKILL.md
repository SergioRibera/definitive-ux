---
name: art-direction
description: 'Art direction for surfaces where the design is the product: landing pages, marketing sites, portfolios, launch screens, onboarding, product pages, campaign pages, in any stack (web, Rust web with Leptos or Dioxus, or a native app''s showcase screens). Replaces the generic default (centered headline, three icon cards, gradient) with decisions: an inventory of the subject, an argument per section, a composed direction, a type and color system from the subject, real imagery, one signature motion, then a look-and-fix loop on the render. Use when the user asks to build or redesign a site, landing, hero, portfolio or marketing page, or says a page looks generic, templated or AI-made. Usually invoked by the definitive-ux router; can be called directly.'
---

# Art direction

> **Stack:** load `stack-map`. Tells and the purpose test come from `definitive-ux/references/slop-tells.md`; tokens go through `design-source`. Method adapted in prose from taste-skill (its study of 247 gallery sites) and impeccable's brand register; no scripts, no asset library.

A page built by a model is the average of every page it has seen: nothing wrong, nothing decided. This skill makes decisions and writes them down before any markup.

## When it applies

Persuade register only: the visitor came to be convinced. Operate and read surfaces (dashboards, settings, docs) follow `better-interface` and `design-review` with the quiet rules in `slop-tells.md`; a product site that shows its app is persuade, and the app fragments on it follow the app-screen rules for their insides.

## The run

### 1. Read the brief and inventory the subject

What is real: the subject, the audience, the one thing the page must get someone to do, the stack, the brand that exists (colors, fonts, logo), and above all what imagery truly exists (own photographs, product shots, a real interface, an image tool, or nothing). Never plan around pictures you do not have.

Then list eight to twelve things that belong to this subject and to no competitor: what the name means; the street, city, materials and light of its world; the numbers in the brief (seats, minutes, price, year); the constraint it lives with; its rituals and hours; the printed things of its world (a ticket, a label, a timetable). The idea comes from this list. If the request is vague, decide and state your reading in one line; ask only when a missing fact blocks the work.

### 2. Write the argument

One line per section: the claim and its proof. The claims decide which sections exist and in what order. Delete any section with no claim. This is how the stock sequence (hero, logos, three features, testimonial, pricing, CTA) never appears by default.

### 3. Choose a direction

Write three candidate directions in two lines each: the idea taken from the inventory, the composition that carries it (a full-bleed photograph with the title set into it; a typographic page with one enormous word; a catalogue of real objects; a timetable; a letter), the type class (grotesk, humanist, serif, slab, display + text), the palette as proportions (one ground, one ink, one accent, fields) with colors taken from the subject, and the signature motion (one). Choose the one that only this subject could own; write one line on why. Then fix the direction sheet: type scale, palette values, hero composition, a blueprint per section of the argument, imagery plan.

Adopt it whole. Redraw one slot only for a reason you can state (a brand constraint, an asset that does not exist). Never redraw toward the familiar.

### 4. Imagery

Real pictures of this subject's world: the project's own, or open archives and museums credited, or a real interface fragment built in the stack with real data. A product with no photographs is drawn or set among real pictures of its world, never a fake product shot. At least four real pictures on a page; crops share ratios; every image does a job (shows the thing, sets the place, proves the claim). Grey boxes, stretched photos and fake dashboards made of bars are the weakest pictures a page can have.

### 5. Build

Tokens through `design-source` (the direction's palette and type become the semantic layer). Semantic structure, visible focus, labels, reduced motion, responsive from 320px. The hero carries the idea; every later section carries its claim with its full content. Type set large; whole fields of color; sections separated by space and tone, not hairlines. One authored motion moment, exponential ease-out, nothing on every section.

### 6. Look, then fix

Render at three widths (mobile, tablet, desktop) and judge the screenshots, never the code. Fourteen passes, each on its own:

1. One second, then five: what did you see? There must be one answer. Can a stranger say what this is, who it is for and what to do next?
2. Squint: two or three large masses with clear size differences, or many medium ones?
3. Cover the logo: could this belong to a competitor? Does the idea reach the pricing, the FAQ and the footer?
4. Edges: count distinct left edges of text blocks; two or three reused on purpose, not six.
5. Space and frame: related close, unrelated far; gaps between sections larger than any inside; no accidental holes; left and right margins equal; no section leaving half its width empty; no stretch of nothing taller than half a screen.
6. Type: body-to-headline jump decisive (3× or more); five sizes at most; no orphan word; no paragraph over 75 characters or centered over three lines.
7. Color: which holds most of the surface, which supports, where is the accent; the accent appears on the single action and one detail.
8. Images: at least four real ones, every one doing a job, crops sharing ratios, nothing grey or stretched.
9. Tells: run `slop-tells.md` on the render.
10. Controls: every button named for its action; one dominant action per view.
11. States: hover, focus, loading, error, empty exist.
12. Mobile: the same idea survives at 320px; nothing stacks into a generic column.
13. Motion: one moment, earned, reduced-motion branch present.
14. Substance: did removing the tells leave a hole? Add type size, a field of color, a real picture, full content.

Fix the worst finding, render again, stop when the one-second pass gives one answer and the tells pass is clean.

## Output

The direction sheet (kept in `design/direction.md`), the argument, the built page, the credits for any found imagery, and a UX decisions note (3–6 bullets, decision + law). Then the router's review loop runs `design-review` on the result.
