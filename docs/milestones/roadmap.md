---
title: "Roadmap & Milestones — vom Walking Skeleton bis Mobile"
---
# Roadmap & Milestones — vom Walking Skeleton bis Mobile

M1 bis M4 sind der Prototyp ([[spec-001-prototype|Spec 001]]). Danach kommen die großen Erweiterungen, jede auf einer Grundlage, die der Prototyp schon legt. Die konkreten nächsten Aufgaben stehen im [[backlog|Backlog]].

| Milestone | Inhalt | Abnahme |
| --- | --- | --- |
| M1 — Walking Skeleton | Workspace (`mage_sim`, `mage_client`), CI für alle Plattformen (`cargo check`), Test-Support, IDs inkl. `TeamId`, Tick-Kette mit Avian in `FixedUpdate` ([[p-005-avians-physik-in-fixedupdate-einhaengen\|P-005]]), Intent, Attribute, RNG, Mana, Cooldowns, Bolt, 2-gegen-2, Snapshot, Determinismustest, Benchmark-Gerüst | alle Tests grün; `mage_sim` baut für alle Ziele; `cargo run` zeigt noch nichts |
| M2 — Zauber-Matrix | Sichtbarkeit (Grundform), Beobachtung, Homing mit Zielwahl nach Team und Sichtbarkeit, Slow als Attribut-Modifikator, Feuerball (GroundArea, Split, AreaDamage, Fragmente), Tod, Respawn | alle drei Zauber simulierbar; Tod/Respawn- und Sichtbarkeitstests grün |
| M3 — Arena & Präsentation | Greybox, Orbitkamera, Low-Res-Target, Interpolation, HUD aus der Beobachtung, Eingabe über Aktionen (Tastatur/Maus und Controller), Englisch und Deutsch, Animation entkoppelt | `cargo run` spielbar gegen Dummies, mit Maus oder Controller, auf Englisch oder Deutsch |
| M4 — Härtung & Messung | Determinismustest über alle Zauber, Benchmarks vollständig (Simulation und Bildzeit), Referenzrechner, Desktop-Builds für Windows, Linux, macOS in CI, Clippy null, README, Vault-Logs | alle Abnahmekriterien von Spec 001; Budgets auf dem Referenzrechner gemessen |
| M5 — Unsichtbarkeit | Tarnung und Wahrnehmung als Attribute, Unsichtbarkeits-Zauber, Enthüllung, Darstellung ([[sichtbarkeit\|Sichtbarkeit]]) | Sichtbarkeitstests inkl. „keine Information durch Wände“; Benchmark `stealth_5v5` im Budget |
| M6 — KI & Reinforcement Learning | regelbasierte KI-Gegner, RL-Umgebung, Brücke zu Python ([[p-013-rl-bruecke\|P-013]]), erstes Selbstspiel-Balancing ([[ki-und-rl\|KI & RL]]) | KI-Gegner im Spiel; Training läuft schneller als Echtzeit |
| M7 — Netzwerk & Sicherheit | Server, Client-Vorhersage, Interest Management, Transport ([[p-009-netzwerk-crate-und-transport\|P-009]]), Sicherheitsmaßnahmen ([[sicherheit\|Sicherheit]]), Sync- und Latenztests ([[synchronisation-latenz\|Synchronisation & Latenz]]) | Teams über das Netz spielbar; alle Sync-/Latenztests grün |
| M8 — Add-ons | Sandbox, Add-on-Schnittstelle, Spielmodus-Regeln ([[add-ons\|Add-ons]], [[p-010-add-on-laufzeit-und-fairness\|P-010]]) | Beispiel-Add-ons (Auslesen und Automatisieren) laufen auf allen Desktop-Plattformen |
| M9 — Mobile & Web | Android- und iOS-Pakete, Touch-Steuerung, Web-Client wenn machbar ([[plattformen\|Plattformen]], [[p-014-mobile-und-web-builds\|P-014]]) | spielbar auf einem Android- und einem iOS-Gerät |

Die Reihenfolge ab M5 ist ein Vorschlag; KI (M6) kommt vor dem Netzwerk, weil sie nur die Beobachtung braucht und früh Spielpartner und Balance-Daten liefert.

Implementation-Logs („Milestone N — Implementation Log“, `milestones/mN-implementation-log.md`) werden je Milestone ergänzt, sobald gebaut wird. Erledigte Backlog-Items wandern dorthin.
