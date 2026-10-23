#!/usr/bin/env bash
# UserPromptSubmit hook: if the prompt smells like UI/UX work, remind Claude to
# route through the definitive-ux orchestrator. Prints nothing otherwise.
set -u
input="$(cat 2>/dev/null || true)"
prompt="$(printf '%s' "$input" | sed -n 's/.*"prompt"[[:space:]]*:[[:space:]]*"\(.*\)".*/\1/p' | head -c 4000)"
[ -z "$prompt" ] && prompt="$input"
lower="$(printf '%s' "$prompt" | tr '[:upper:]' '[:lower:]')"

# Spanish + English signals. Word boundaries where a short token would be noisy.
pattern='(\bui\b|\bux\b|interfa[cz]|dise[ñn]|design|redesign|redise|pantalla|screen|layout|componente|component|bot[oó]n|button|formulario|\bform\b|onboarding|landing|pricing|precios|dashboard|\bweb\b|sitio|website|p[aá]gina|\bpage\b|app m[oó]vil|mobile app|desktop app|app de escritorio|aplicaci[oó]n|usabilidad|usability|accesib|accessib|animaci|animation|motion|spacing|espaciado|padding|flujo de usuario|user flow|user stor|wireframe|mockup|figma|penpot|freya|\begui\b|eframe|\bgpui\b|slint|\biced\b|dioxus|leptos|\byew\b|ratatui|tauri|electron|swiftui|compose|flutter|react native|\btv\b|android tv|reloj|smartwatch|wear os|embebid|embedded|kiosk|\bhmi\b|tokens|design system|dark pattern|nativo|native feel|genérica|generic|slop|frontend|front-end|\btema\b|theme|dark mode|modo oscuro|iconos|icons|tipograf|typograph|empty state|estado vac|loading state|modal|tooltip|dark pattern|checkout|navegaci[oó]n|navigation|menu|sidebar|settings|ajustes|hick|fitts|laws of ux|leyes de ux|heur[ií]stic|dise[ñn]o de producto|product design)'

if printf '%s' "$lower" | grep -Eq "$pattern"; then
  cat <<'MSG'
[definitive-ux] This prompt looks like UI/UX work. Before answering, load the `definitive-ux:definitive-ux` skill and follow its router: classify the intent (question / build / redesign / audit / polish / bolder / quieter / distill / harden / motion / spacing / components / design source / flows / stress test), detect the stack and form factor via `stack-map` (desktop, mobile, tv, watch, embedded, web, tui), then invoke only the specialist skills the router names, in the order it gives. Produce one consolidated report, not one per skill.
MSG
fi
exit 0
