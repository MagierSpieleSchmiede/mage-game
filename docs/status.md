---
title: "Status — was existiert, was fehlt"
---
# Status — was existiert, was fehlt

Stand 2026-11-14 (Vault v1.2): spezifiziert und geplant, nichts implementiert. ADR-0017 (Rollback mit Server als Schiedsrichter) und ADR-0027 (Zielfilter pro Fähigkeit) sind angenommen; die übrigen ADRs von ADR-0015 bis ADR-0026 sind **vorgeschlagen** und warten auf Freigabe. Die nächsten Aufgaben stehen im [[backlog|Backlog]].

| Bereich | Heute | Fehlt |
| --- | --- | --- |
| Simulation | vollständig spezifiziert (Kette, Intent, Zauber, Kampf, Determinismus) | Implementierung (ab M1) |
| Teams und Attribute | geplant ([[teams\|Teams]], [[attribute\|Attribute]]) | Friendly-Fire-Regel ([[p-007-friendly-fire-und-team-regeln\|P-007]]); Implementierung (M1) |
| Sichtbarkeit und Unsichtbarkeit | geplant ([[sichtbarkeit\|Sichtbarkeit]], [[beobachtung\|Beobachtung]]) | Grundform (M2), Tarnung (M5) |
| Präsentation | spezifiziert (Kamera, Render-Target, HUD, Interpolation, Animation entkoppelt) | Implementierung (M3) |
| Eingabe | geplant: Tastatur/Maus, Controller, Touch über Aktionen ([[eingabegeraete\|Eingabegeräte]]) | Crate-Frage ([[p-011-eingabe-crate-und-zielhilfe\|P-011]]); M3 |
| Sprachen | geplant: Englisch und Deutsch ([[lokalisierung\|Lokalisierung]]) | Integrationsfrage ([[p-012-fluent-integration\|P-012]]); M3 |
| Zauber | drei Blueprints definiert (Bolt, Homing-Bolt, Feuerball) | Implementierung, Balance |
| Tests | Strategie mit Tests zuerst, Sync- und Latenztests, Benchmarks geplant ([[teststrategie\|Teststrategie]]) | Implementierung |
| Leistung | Szenarien und Platzhalter-Budgets ([[benchmarks\|Benchmarks]]) | Referenzrechner ([[p-008-mindest-hardware-und-leistungsbudget\|P-008]]); M4 |
| Plattformen | Stufen und Regeln ([[plattformen\|Plattformen]]) | CI-Ziele ([[p-014-mobile-und-web-builds\|P-014]]) |
| KI und RL | geplant ([[ki-und-rl\|KI & RL]]) | Brücke ([[p-013-rl-bruecke\|P-013]]); M6 |
| Netzwerk und Sicherheit | geplant: Rollback-Netcode mit Server als Schiedsrichter ([[netzwerk\|Netzwerk]], [[sicherheit\|Sicherheit]]) | Transport ([[p-009-netzwerk-crate-und-transport\|P-009]]), Trefferwertung ([[p-015-trefferwertung-unter-latenz\|P-015]]); M7 |
| Add-ons | geplant ([[add-ons\|Add-ons]]) | Laufzeit und Fairness ([[p-010-add-on-laufzeit-und-fairness\|P-010]]); M8 |
| Gestaltung / Assets | Greybox geplant; Stil-Vorbild; Einstieg für Gestaltende ([[einstieg-gestalten\|Gestalten]]) | Asset-Ablage ([[p-006-asset-ablage-und-grosse-binaerdateien\|P-006]]) |

Offene technische Fragen stehen im [[problem-log|Problem-Log]].
