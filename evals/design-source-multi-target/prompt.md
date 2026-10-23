---
tags: [definitive-ux]
max_turns: 30
timeout_seconds: 900
allowed_tools: [Read, Glob, Grep, Skill, Write, Edit, Bash]
---

tengo un producto "Tally" (punto de venta para tiendas chicas) con tres apps en este workspace: la de escritorio en Freya, la terminal HMI en Slint (800x480, touch resistivo, local con mucha luz) y la web en Leptos. Cada una tiene sus colores y tamaños puestos a mano y se ven como tres productos distintos. Quiero una sola fuente de diseño para que las tres se sientan el mismo producto y nativas en su plataforma. Armámela.

Input files: Cargo.toml, crates/desktop/**, crates/hmi/**, crates/web/**
