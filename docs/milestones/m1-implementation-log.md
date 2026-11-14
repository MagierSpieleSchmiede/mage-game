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
| B-003 | 2026-11-14 | `mage_sim`: `TICKS_PER_SECOND` (60), `MageSimPlugin` (Fixed-Takt), Szenario-Builder mit Teams, Spielern, Seed und Prüfung (2 bis 32 Teams, kein leeres Team), `headless::headless_app` und `step_ticks`. Anders als geplant: `TimeUpdateStrategy::FixedTimesteps(1)` statt `ManualDuration`, und ein Anlauf-Update, weil das erste Update keinen Tick ausführt ([[headless-integrationstest\|Headless-Integrationstest]]). ADR-0015 angenommen. |
| B-004 | 2026-11-15 | `ids`: `SimId` (u64) mit `SimIdCounter`, `PlayerId` (u32), `TeamId` (u8), `SpellId` (u16); `mage::spawn_scenario_mages` spawnt beim Start einen Magier pro Spieler, Team für Team, Spieler ab 0 nummeriert. Anders als geplant: `SimId` und `PlayerId` breiter als im Vault, wegen der großen Welt ([[adr-0028-grosse-welt-mit-interessenbereich\|ADR-0028]]); serde erst mit dem Snapshot (B-013). |
| B-005 | 2026-11-15 | `schedule::SimSet` mit den neun Sets, verkettet in `FixedUpdate`; Avian per `PhysicsPlugins::new(FixedUpdate)` im Set Physik. Der Test zeigte zuerst, dass Avians Standard-Schedule P-005 tatsächlich auslöst; P-005 erledigt. Anders als geplant: `headless_app` braucht `TaskPoolPlugin`, `TransformPlugin` sowie `app.finish()` und `app.cleanup()` ([[headless-integrationstest\|Headless-Integrationstest]]). |
| B-006 | 2026-11-15 | `intent`: `MAX_SLOTS` (12), `MageIntent`, `IntentForTick` mit `just_released`, `Controller` (Spieler oder KI, bringt `IntentForTick` mit), `InputBuffer` nach `SimId` mit Latch-Zählern, `freeze_intents` im Set Intent mit Bereinigung von Achse, Zielrichtung und Slot-Nummern. Magier werden mit `Controller::Player` gespawnt. Ein Gegentest ohne Latch-Rücksetzen ließ die Flanken-Tests scheitern. |
