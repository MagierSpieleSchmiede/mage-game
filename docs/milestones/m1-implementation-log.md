---
title: "Milestone 1 — Implementation Log"
---
# Milestone 1 — Implementation Log

Was in M1 erledigt wurde, in der Reihenfolge des Abschlusses. Ein Satz pro Backlog-Item, mit Verweis auf neue ADRs und P-Einträge.

| Item | Erledigt am | Ergebnis |
| --- | --- | --- |
| B-010 | 2026-11-14 | Friendly Fire wird pro Fähigkeit über einen Zielfilter entschieden ([[adr-0027-zielfilter-pro-faehigkeit\|ADR-0027]]); P-007 geschlossen. |
| B-001 | 2026-11-14 | Workspace mit `crates/mage_sim` (Bevy nur `std`, Avian nur `3d` und `parry-f32`) und `crates/mage_client` (leer); ein Test prüft per `cargo tree` für alle Ziele, dass `mage_sim` keine Fenster-, Render- oder Audio-Crate erreicht und auf `bevy_ecs` und `avian3d` baut. `mage_sim` baut für `wasm32-unknown-unknown`. |
| B-002 | 2026-11-14 | CI (`.github/workflows/ci.yml`): fmt, clippy, Tests auf Linux, Windows, macOS; `cargo check -p mage_sim` für wasm32, Android, iOS; grün auf allen Zielen, und rot bei einem absichtlichen Clippy-Fehler (Wegwerf-Branch). Anders als geplant: Android braucht das Feature `android-check`, iOS einen macOS-Runner ([[plattformen\|Plattformen]]). ADR-0024 angenommen. |
