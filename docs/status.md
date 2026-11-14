---
title: "Status — was existiert, was fehlt"
---
# Status — was existiert, was fehlt

Die eine Seite zum aktuellen Planungsstand. README, `AGENTS.md` und die Startseite verweisen hierher, statt ihn zu wiederholen.

## Stand

**2026-11-14, Vault v1.3:** spezifiziert und geplant, nichts implementiert; `cargo run` gibt nur „Hello, world!“ aus.

| Frage | Antwort |
| --- | --- |
| Was ist der nächste Schritt? | Milestone 1 — Walking Skeleton, beginnend mit **B-001** (Workspace) im [[backlog\|Backlog]]. Gestartet wird, wenn Daniel es freigibt. |
| Was blockiert den Start? | Freigabe der vorgeschlagenen ADRs, die M1 braucht, vor allem ADR-0020, ADR-0026, ADR-0015 und ADR-0016 ([[entscheidungen\|Entscheidungen]]). |
| Welche Entscheidungen gelten? | Alle angenommenen ADRs; welche das sind, steht nur in [[entscheidungen\|Entscheidungen]]. |
| Welche Fragen sind offen? | [[problem-log\|Problem-Log]]; für M1 relevant ist [[p-005-avians-physik-in-fixedupdate-einhaengen\|P-005]] (Avian in `FixedUpdate`). |
| Was ist schon erledigt? | Implementation-Logs je Milestone, derzeit [[m1-implementation-log\|M1]]. |

## Bereich für Bereich

| Bereich | Heute | Fehlt |
| --- | --- | --- |
| Simulation | vollständig spezifiziert (Kette, Intent, Zauber, Kampf, Determinismus) | Implementierung (ab M1) |
| Teams und Attribute | geplant ([[teams\|Teams]], [[attribute\|Attribute]]); Friendly Fire pro Fähigkeit entschieden ([[adr-0027-zielfilter-pro-faehigkeit\|ADR-0027]]) | Implementierung (M1) |
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

## Verlauf

| Version | Datum | Änderung |
| --- | --- | --- |
| v1.3 | 2026-11-14 | Vault entdoppelt: ADR-Stand nur noch in [[entscheidungen\|Entscheidungen]], Planungsstand nur hier, Arbeitsablauf nur im [[backlog\|Backlog]], Nummernzähler je an einer Stelle. |
| v1.2 | 2026-11-14 | Teams, Attribute, Unsichtbarkeit, KI und RL, Netzwerk und Sicherheit, Add-ons, Plattformen, Sprachen, Eingabegeräte, Benchmarks geplant; ADR-0017 und ADR-0027 angenommen. |
