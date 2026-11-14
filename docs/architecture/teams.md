---
title: "Teams — zwei oder mehr Teams mit beliebig vielen Magiern"
---
# Teams — zwei oder mehr Teams mit beliebig vielen Magiern

Mage Game ist kein festes Duell: Eine Runde hat zwei oder mehr Teams, jedes mit einem oder mehreren Magiern (1-gegen-1, 2-gegen-2, 3-gegen-3-gegen-3 …). Entschieden in [[adr-0015-teams-statt-fester-zwei-spieler|ADR-0015]] (vorgeschlagen).

## Was gilt

- Jeder Magier trägt `PlayerId` (wer) und `TeamId` (zu wem er gehört). Beide sind stabile IDs, nie `Entity` ([[serialisierung-ids|Serialisierung & IDs]]).
- „Gegner“ heißt immer: Magier mit anderer `TeamId`. Kein Code fragt nach „dem“ Gegner.
- **Zielwahl** (Homing): der nächste Gegner, der für das eigene Team sichtbar ist ([[sichtbarkeit|Sichtbarkeit]]); bei gleichem Abstand entscheidet die kleinere `SimId`.
- **Spawnpunkte** gehören zu Teams; die Arena definiert pro Team mehrere.
- **Rundenende** (später): ein Team bleibt übrig, oder Punkte nach Zeit. Das ist Spielmodus-Logik und kommt nach dem Prototyp.
- Eine Runde wird durch ein **Szenario** beschrieben: Teams, Spieler pro Team, Arena, Seed. Tests, Benchmarks, KI-Training und später das Netzwerk-Lobby benutzen dasselbe Szenario-Format.

## Was offen ist

- Treffen eigene Zauber Verbündete? [[p-007-friendly-fire-und-team-regeln|P-007]].
- Teamfarben im HUD und an den Magiern, auch für Farbenblinde: [[stil-vorbild|Stil-Vorbild]].
- Obergrenze an Spielern pro Runde: ergibt sich aus [[benchmarks|Benchmarks]] und der Bandbreite ([[netzwerk|Netzwerk]]).
