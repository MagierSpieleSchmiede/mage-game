---
title: "Mage Game"
description: Ein stilisiertes 3D-Arena-Spiel für Teams von Magiern in Rust mit Bevy und Avian — Spezifikation, Architektur, Entscheidungen und offene Probleme des Prototyps.
---
# Mage Game

![[vorbild.jpg|Stil-Vorbild: ein Magier mit leuchtendem Kristallstab bei Nacht, Vollmond über einem See mit Burg und Dorf]]

**Mage Game** ist ein stilisiertes 3D-Arena-Spiel in Rust (Bevy 0.20, Avian 0.8): zwei oder mehr Teams von Magiern, freie 3D-Bewegung, Third-Person-Zielen, modulare Zauber und Eigenschaften, die sich im Kampf ändern. Geplant für Windows, Linux, macOS, Android und iOS, auf Englisch und Deutsch, mit Tastatur und Maus oder Controller. Später kommen PvP über einen Server, Unsichtbarkeit, KI-Gegner aus Reinforcement Learning, Add-ons, Kreaturen (Tiere, Pets, Bosse), Reittiere und eine große Welt für tausende Spieler. Dieser Vault dokumentiert den fundamentalen Prototyp: Spezifikation, Architektur, Entscheidungen, Teststrategie und offene Probleme, eine Seite pro Thema.

> [!info] Stand
> Geplant, noch nicht implementiert. Was zuletzt erledigt wurde und was als Nächstes kommt: **[[status|Status]]**. Die nächsten Aufgaben: **[[backlog|Backlog]]**.

> [!tip] Neu hier?
> **[[einstieg-windows|Einstieg für Mitwirkende (Windows)]]** führt von null bis zum ersten Beitrag: Git, GitHub Desktop, Rust, Obsidian. Danach: **[[einstieg-ki-agenten|KI-Agenten]]** (Claude Code, Codex) für alle, die mit einem Agenten arbeiten, und **[[einstieg-gestalten|Gestalten]]** (Blender, ArtCraft) für alle, die am Look des Spiels arbeiten.

## Hier anfangen

1. [[status|Status]] — die Übergabe: was zuletzt erledigt wurde, was als Nächstes kommt
2. [[uebersicht|Übersicht]] — das System in einem Diagramm
3. [[projektstruktur|Projektstruktur]] — das Plugin-Layout und warum jedes Plugin existiert
4. [[simulationsmodell|Simulationsmodell]] — die eine große Wette: eine deterministische Tick-Kette
5. [[zauber-datenmodell|Zauber-Datenmodell]] — Blueprints, Deliveries, Modifikatoren, Effekte
6. [[backlog|Backlog]] — die nächsten Aufgaben, mit Abhängigkeiten
7. [[roadmap|Roadmap & Milestones]] und [[erweiterungspunkte|Erweiterungspunkte]] — was nach dem Prototyp kommt
8. [[entscheidungen|Entscheidungen]] — alle ADRs mit ihrem Stand
9. [[sicherheit|Sicherheit]] — wie Server und PvP abgesichert werden

## Bereiche

| Bereich | Seiten |
| --- | --- |
| Architektur | [[uebersicht\|Übersicht]] · [[projektstruktur\|Projektstruktur]] · [[simulationsmodell\|Simulationsmodell]] · [[intent-steuerung\|Intent & Steuerung]] · [[zauber-datenmodell\|Zauber-Datenmodell]] · [[kampf-physik\|Kampf & Physik]] · [[arena-praesentation\|Arena & Präsentation]] · [[determinismus\|Determinismus]] · [[serialisierung-ids\|Serialisierung & IDs]] · [[teams\|Teams]] · [[attribute\|Attribute]] · [[sichtbarkeit\|Sichtbarkeit]] · [[beobachtung\|Beobachtung]] · [[eingabegeraete\|Eingabegeräte]] · [[faehigkeiten\|Fähigkeiten & Belegung]] · [[einheiten\|Einheiten]] · [[lokalisierung\|Lokalisierung]] · [[plattformen\|Plattformen]] |
| Später | [[netzwerk\|Netzwerk]] · [[sicherheit\|Sicherheit]] · [[ki-und-rl\|KI & RL]] · [[add-ons\|Add-ons]] · [[reittiere\|Reittiere]] · [[welt\|Welt]] |
| Entscheidungen | [[entscheidungen\|Entscheidungen]] — alle ADRs mit ihrem Stand |
| Spezifikation | [[spec-001-prototype\|Spec 001 — fundamentaler Prototyp]] |
| Testing | [[teststrategie\|Teststrategie]] · [[headless-integrationstest\|Headless-Integrationstest]] · [[determinismustest\|Determinismustest]] · [[synchronisation-latenz\|Synchronisation & Latenz]] · [[benchmarks\|Benchmarks]] |
| Planung | [[status\|Status]] · [[backlog\|Backlog]] · [[roadmap\|Roadmap & Milestones]] · [[erweiterungspunkte\|Erweiterungspunkte]] |
| Probleme | [[problem-log\|Problem-Log]] |
| Konventionen | [[konventionen\|Konventionen]] · [[adr-nnnn\|ADR-Vorlage]] · [[p-nnn\|P-Vorlage]] |
| Gestaltung | [[stil-vorbild\|Stil-Vorbild]] · [[einstieg-gestalten\|Gestalten]] |
| Glossar | [[glossar\|Glossar]] — Begriffe von Intent bis Rollback |
| Mitmachen | [[einstieg-windows\|Einstieg für Mitwirkende (Windows)]] · [[einstieg-ki-agenten\|KI-Agenten]] · [[einstieg-gestalten\|Gestalten]] |
