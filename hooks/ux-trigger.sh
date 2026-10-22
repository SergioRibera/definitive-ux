#!/usr/bin/env bash
# UserPromptSubmit hook: if the prompt smells like UI/UX work, remind Claude to
# route through the native-ux orchestrator. Prints nothing otherwise.
set -u
input="$(cat 2>/dev/null || true)"
prompt="$(printf '%s' "$input" | sed -n 's/.*"prompt"[[:space:]]*:[[:space:]]*"\(.*\)".*/\1/p' | head -c 4000)"
[ -z "$prompt" ] && prompt="$input"
lower="$(printf '%s' "$prompt" | tr '[:upper:]' '[:lower:]')"

# Spanish + English signals. Word boundaries where a short token would be noisy.
pattern='(\bui\b|\bux\b|interfa[cz]|dise[ñn]|design|redesign|redise|pantalla|screen|layout|componente|component|bot[oó]n|button|formulario|\bform\b|onboarding|landing|pricing|precios|dashboard|\bweb\b|sitio|website|p[aá]gina|\bpage\b|app m[oó]vil|mobile app|desktop app|app de escritorio|aplicaci[oó]n|usabilidad|usability|accesib|accessib|animaci|animation|motion|spacing|espaciado|padding|flujo de usuario|user flow|user stor|wireframe|mockup|figma|penpot|freya|\begui\b|eframe|\bgpui\b|frontend|front-end|\btema\b|theme|dark mode|modo oscuro|iconos|icons|tipograf|typograph|empty state|estado vac|loading state|modal|tooltip|dark pattern|checkout|navegaci[oó]n|navigation|menu|sidebar|settings|ajustes|hick|fitts|laws of ux|leyes de ux|heur[ií]stic|dise[ñn]o de producto|product design)'

if printf '%s' "$lower" | grep -Eq "$pattern"; then
  cat <<'MSG'
[native-ux] This prompt looks like UI/UX work. Before answering, load the `native-ux:native-ux` skill and follow its router: classify the intent (question / build / redesign / audit / motion / spacing / components / flows), detect the stack (Rust native via `rust-gui-map`, web, mobile, description-only), then invoke only the specialist skills the router names, in the order it gives. Produce one consolidated report, not one per skill.
MSG
fi
exit 0
