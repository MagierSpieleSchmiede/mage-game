---
title: "Mage Game"
description: Ein stilisiertes 3D-Arena-Duellspiel in Rust mit Bevy und Avian — Spezifikation, Architektur, Entscheidungen und offene Probleme des Prototyps.
---
# Mage Game

**Mage Game** ist ein stilisiertes 3D-Arena-Duellspiel in Rust (Bevy 0.20, Avian 0.8). Spieler steuern Magier mit freier 3D-Bewegung, Third-Person-Zielen und modularen, anpassbaren Zaubern; später kommen PvP mit Rollback-Netcode und KI-Gegner. Dieser Vault dokumentiert den fundamentalen Prototyp: Spezifikation, Architektur, Entscheidungen, Teststrategie und offene Probleme, eine Seite pro Thema.

> [!info] Status — Vault v1.1 (2026-11-14)
> Spezifikation nach Review überarbeitet; [[p-001-move-and-slide|P-001]] und [[p-002-observer-laufzeitpunkt-innerhalb-der-tick-kette|P-002]] sind entschieden, offen bleiben P-003 bis P-005. Implementierung noch nicht begonnen. Nächster Schritt: Milestone 1 — Walking Skeleton. Alles Bereich für Bereich: **[[status|Status]]**.

> [!tip] Neu hier?
> **[[einstieg-windows|Einstieg für Mitwirkende (Windows)]]** führt von null bis zum ersten Pull Request: Git, GitHub Desktop, Rust, Obsidian.

## Hier anfangen

1. [[status|Status]] — was existiert, was fehlt
2. [[uebersicht|Übersicht]] — das System in einem Diagramm
3. [[projektstruktur|Projektstruktur]] — das Plugin-Layout und warum jedes Plugin existiert
4. [[simulationsmodell|Simulationsmodell]] — die eine große Wette: eine deterministische Tick-Kette
5. [[zauber-datenmodell|Zauber-Datenmodell]] — Blueprints, Deliveries, Modifikatoren, Effekte
6. [[roadmap|Roadmap & Milestones]] und [[erweiterungspunkte|Erweiterungspunkte]] — was nach dem Prototyp kommt

## Bereiche

| Bereich | Seiten |
| --- | --- |
| Architektur | [[uebersicht\|Übersicht]] · [[projektstruktur\|Projektstruktur]] · [[simulationsmodell\|Simulationsmodell]] · [[intent-steuerung\|Intent & Steuerung]] · [[zauber-datenmodell\|Zauber-Datenmodell]] · [[kampf-physik\|Kampf & Physik]] · [[arena-praesentation\|Arena & Präsentation]] · [[determinismus\|Determinismus]] · [[serialisierung-ids\|Serialisierung & IDs]] |
| Entscheidungen | [[adr-0001-tick-simulation-in-fixedupdate-statt-delta\|ADR-0001]] bis [[adr-0014-impacts-sammeln-dann-sortiert-ausloesen\|ADR-0014]], alle im Ordner `decisions/` |
| Spezifikation | [[spec-001-prototype\|Spec 001 — fundamentaler Prototyp]] |
| Testing | [[teststrategie\|Teststrategie]] · [[headless-integrationstest\|Headless-Integrationstest]] · [[determinismustest\|Determinismustest]] |
| Planung | [[status\|Status]] · [[roadmap\|Roadmap & Milestones]] · [[erweiterungspunkte\|Erweiterungspunkte]] |
| Probleme | [[problem-log\|Problem-Log]] (P-001 bis P-005) |
| Konventionen | [[konventionen\|Konventionen]] · [[adr-nnnn\|ADR-Vorlage]] · [[p-nnn\|P-Vorlage]] |
| Glossar | [[glossar\|Glossar]] — Begriffe von Intent bis Rollback |
| Mitmachen | [[einstieg-windows\|Einstieg für Mitwirkende (Windows)]] |
