---
title: "Teststrategie — Tests zuerst, auf sieben Ebenen"
---
# Teststrategie — Tests zuerst, auf sieben Ebenen

**Tests zuerst** ([[adr-0026-tests-zuerst|ADR-0026]]): Jedes [[backlog|Backlog]]-Item beginnt mit den Tests aus seiner Spalte *Tests*; der Ablauf steht im Backlog. Diese Seite sagt, welche Arten von Tests es gibt.

| Ebene | Was | Wie | Ab |
| --- | --- | --- | --- |
| 1. Unit | Flanken-Ableitung, Cooldowns, Attribut-Verrechnung und Stapelregeln, Blueprint-Lookup, Zielwahl | reine Funktionen, keine App | M1 |
| 2. Headless-Integration | Abläufe über mehrere Ticks: Bolt trifft, 2-gegen-2, Tod und Respawn, Sichtbarkeit | App nur aus `mage_sim`, Ticks per `step_ticks` ([[headless-integrationstest\|Headless-Integrationstest]]) | M1 |
| 3. Determinismus | gleicher Seed und gleiche Intents → byte-gleicher Zustand | zwei Läufe, serialisierter Vergleich ([[determinismustest\|Determinismustest]]) | M1 |
| 4. Präsentation | Animation blockiert keinen Cast; HUD zeigt nur Beobachtung; alle Texte vorhanden in Englisch und Deutsch | Client-App ohne Fenster bzw. mit Test-Fenster; Sprachdatei-Tests ([[lokalisierung\|Lokalisierung]]) | M3 |
| 5. Leistung | Tick-Kosten, Schritte pro Sekunde, Bildzeit, Speicher | Benchmarks mit Szenarien ([[benchmarks\|Benchmarks]]) | M1 (Gerüst), M4 |
| 6. Synchronisation und Latenz | Rollback-Exaktheit, Server-Korrekturen, Konvergenz, Sichtbarkeitsfilter, Eingabeverzögerung | Server und Clients in einem Prozess mit simulierter Leitung ([[synchronisation-latenz\|Synchronisation & Latenz]]) | M7 |
| 7. Robustheit | fehlerhafte Netzwerkdaten, Add-on-Missbrauch | Fuzz-Tests, Sandbox-Grenztests ([[sicherheit\|Sicherheit]]) | M7, M8 |

## Pflicht-Tests, die nie fehlen dürfen

- **Animation blockiert nichts:** Ein Cast wirkt im Tick seiner Eingabe (bzw. nach seiner Wirkzeit in Ticks), egal ob und welche Animation läuft, und auch ohne Präsentation ([[adr-0021-animation-ist-reine-praesentation|ADR-0021]]).
- **Keine Information durch Wände:** Eine Beobachtung enthält keinen Gegner ohne Team-Bit in `VisibleTo` ([[sichtbarkeit|Sichtbarkeit]]).
- **Kein Eingabeverlust:** Ein Druck geht nie verloren und wird nie doppelt gezählt, bei 0, 1 und mehreren Ticks pro Frame ([[intent-steuerung|Intent & Steuerung]]).
- **Plattform:** `mage_sim` baut für `wasm32-unknown-unknown` ([[plattformen|Plattformen]]).

## Regeln

- Szenarien (Teams, Spieler, Arena, Seed) werden mit einem Builder aus dem Test-Support gebaut; Tests, Benchmarks und RL nutzen dieselben Szenarien.
- Kein Test liest die Uhr; alles läuft in Ticks.
- Ein noch nicht erfüllbarer Test ist `#[ignore = "B-nnn"]` mit Verweis auf sein Backlog-Item, nie auskommentiert.
