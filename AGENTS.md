# definitive-ux

Agent Skills repository. Every skill lives in `skills/<name>/SKILL.md` (frontmatter `name` + `description`, body with the rules) and optional `references/` and `agents/openai.yaml`.

Working on this repo:

- Edit the skill body, never only the README. The description is the trigger; keep it specific and a little pushy.
- Framework API facts go only in `skills/rust-gui-map/SKILL.md`; web and mobile idioms in `skills/definitive-ux/references/web-map.md`.
- New use cases: add a router row in `skills/definitive-ux/SKILL.md` and a playbook in `skills/definitive-ux/references/playbooks.md`.
- Evals in `evals/<case>/` follow the `claude plugin eval` layout; add a Rust fixture when a rule is Rust-specific.
- Commits: single-line Conventional Commit.
