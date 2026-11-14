---
title: "Roadmap & Milestones"
---
# Roadmap & Milestones

| Milestone | Inhalt | Abnahme |
| --- | --- | --- |
| M1 — Walking Skeleton | Simulations-Plugins headless: zwei Magier, Intent-Einfrieren, `SimId`, Avian in `FixedUpdate` ([[p-005-avians-physik-in-fixedupdate-einhaengen|P-005]]), Bolt (Projectile, Spread, Damage), Mana, Cooldowns, Headless-Test, Determinismustest mit Byte-Vergleich | beide Tests grün; `cargo run` zeigt noch nichts |
| M2 — Zauber-Matrix | Homing mit LOS-Verlust, Slow, Feuerball (GroundArea, Split, AreaDamage, Fragmente), Tod, Respawn, Observer-Pfade | alle drei Zauber simulierbar; Tod/Respawn-Test grün |
| M3 — Arena & Präsentation | Greybox, Orbitkamera, Low-Res-Target mit Nearest-Upscale, Interpolation, HUD, HumanController mit Tasten 1/2/3 | `cargo run` spielbar gegen den Dummy |
| M4 — Härtung | Determinismustest auf alle drei Zauber ausweiten, Clippy/Warnings null, README, Vault-Logs | alle Abnahmekriterien von Spec 001 |

Implementation-Logs („Milestone N — Implementation Log") werden je Milestone als eigene Seiten ergänzt, sobald gebaut wird.
