---
title: "Mage Game"
description: Ein stilisiertes 3D-Arena-Spiel für Teams von Magiern in Rust mit Bevy und Avian — Spezifikation, Architektur, Entscheidungen und offene Probleme des Prototyps.
---
# Mage Game

![[vorbild.jpg|Stil-Vorbild: ein Magier mit leuchtendem Kristallstab bei Nacht, Vollmond über einem See mit Burg und Dorf]]

**Mage Game** ist ein stilisiertes 3D-Arena-Spiel in Rust (Bevy 0.20, Avian 0.8): zwei oder mehr Teams von Magiern, freie 3D-Bewegung, Third-Person-Zielen, modulare Zauber und Eigenschaften, die sich im Kampf ändern. Geplant für Windows, Linux, macOS, Android und iOS, auf Englisch und Deutsch, mit Tastatur und Maus oder Controller. Später kommen PvP über einen Server, Unsichtbarkeit, KI-Gegner aus Reinforcement Learning und Add-ons. Dieser Vault dokumentiert den fundamentalen Prototyp: Spezifikation, Architektur, Entscheidungen, Teststrategie und offene Probleme, eine Seite pro Thema.

> [!info] Status — Vault v1.2 (2026-11-14)
> Planung erweitert um Teams, Attribute, Unsichtbarkeit, KI und RL, Netzwerk und Sicherheit, Add-ons, Plattformen, Sprachen, Eingabegeräte und Benchmarks; ADR-0015 bis ADR-0026 sind vorgeschlagen. Implementierung noch nicht begonnen. Nächste Aufgaben: **[[backlog|Backlog]]**. Alles Bereich für Bereich: **[[status|Status]]**.

> [!tip] Neu hier?
> **[[einstieg-windows|Einstieg für Mitwirkende (Windows)]]** führt von null bis zum ersten Beitrag: Git, GitHub Desktop, Rust, Obsidian. Danach: **[[einstieg-ki-agenten|KI-Agenten]]** (Claude Code, Codex) für alle, die mit einem Agenten arbeiten, und **[[einstieg-gestalten|Gestalten]]** (Blender, ArtCraft) für alle, die am Look des Spiels arbeiten.

## Hier anfangen

1. [[status|Status]] — was existiert, was fehlt
2. [[uebersicht|Übersicht]] — das System in einem Diagramm
3. [[projektstruktur|Projektstruktur]] — das Plugin-Layout und warum jedes Plugin existiert
4. [[simulationsmodell|Simulationsmodell]] — die eine große Wette: eine deterministische Tick-Kette
5. [[zauber-datenmodell|Zauber-Datenmodell]] — Blueprints, Deliveries, Modifikatoren, Effekte
6. [[backlog|Backlog]] — die nächsten Aufgaben, mit Abhängigkeiten
7. [[roadmap|Roadmap & Milestones]] und [[erweiterungspunkte|Erweiterungspunkte]] — was nach dem Prototyp kommt
8. [[sicherheit|Sicherheit]] — wie Server und PvP abgesichert werden

## Bereiche

| Bereich | Seiten |
| --- | --- |
| Architektur | [[uebersicht\|Übersicht]] · [[projektstruktur\|Projektstruktur]] · [[simulationsmodell\|Simulationsmodell]] · [[intent-steuerung\|Intent & Steuerung]] · [[zauber-datenmodell\|Zauber-Datenmodell]] · [[kampf-physik\|Kampf & Physik]] · [[arena-praesentation\|Arena & Präsentation]] · [[determinismus\|Determinismus]] · [[serialisierung-ids\|Serialisierung & IDs]] · [[teams\|Teams]] · [[attribute\|Attribute]] · [[sichtbarkeit\|Sichtbarkeit]] · [[beobachtung\|Beobachtung]] · [[eingabegeraete\|Eingabegeräte]] · [[lokalisierung\|Lokalisierung]] · [[plattformen\|Plattformen]] |
| Später | [[netzwerk\|Netzwerk]] · [[sicherheit\|Sicherheit]] · [[ki-und-rl\|KI & RL]] · [[add-ons\|Add-ons]] |
| Entscheidungen | [[adr-0001-tick-simulation-in-fixedupdate-statt-delta\|ADR-0001]] bis [[adr-0014-impacts-sammeln-dann-sortiert-ausloesen\|ADR-0014]] (angenommen), [[adr-0015-teams-statt-fester-zwei-spieler\|ADR-0015]] bis [[adr-0026-tests-zuerst\|ADR-0026]] (vorgeschlagen), alle im Ordner `decisions/` |
| Spezifikation | [[spec-001-prototype\|Spec 001 — fundamentaler Prototyp]] |
| Testing | [[teststrategie\|Teststrategie]] · [[headless-integrationstest\|Headless-Integrationstest]] · [[determinismustest\|Determinismustest]] · [[synchronisation-latenz\|Synchronisation & Latenz]] · [[benchmarks\|Benchmarks]] |
| Planung | [[status\|Status]] · [[backlog\|Backlog]] · [[roadmap\|Roadmap & Milestones]] · [[erweiterungspunkte\|Erweiterungspunkte]] |
| Probleme | [[problem-log\|Problem-Log]] (P-001 bis P-015) |
| Konventionen | [[konventionen\|Konventionen]] · [[adr-nnnn\|ADR-Vorlage]] · [[p-nnn\|P-Vorlage]] |
| Gestaltung | [[stil-vorbild\|Stil-Vorbild]] · [[einstieg-gestalten\|Gestalten]] |
| Glossar | [[glossar\|Glossar]] — Begriffe von Intent bis Rollback |
| Mitmachen | [[einstieg-windows\|Einstieg für Mitwirkende (Windows)]] · [[einstieg-ki-agenten\|KI-Agenten]] · [[einstieg-gestalten\|Gestalten]] |
