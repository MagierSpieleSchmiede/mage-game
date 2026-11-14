---
title: "ADR-0032 — Jede Einheit ist außer Kampf, im Kampf oder tot; davon hängt ab, was am Charakter geändert werden darf"
---
# ADR-0032 — Jede Einheit ist außer Kampf, im Kampf oder tot; davon hängt ab, was am Charakter geändert werden darf

**Status:** vorgeschlagen

**Kontext:** Spieler sollen Fähigkeitenleiste, Ausrüstung und Ähnliches ändern können, aber nicht mitten im Kampf ([[adr-0030-faehigkeitenleiste-vom-spieler|ADR-0030]], [[p-020-faehigkeitenleiste-aendern|P-020]]). Dafür braucht die Simulation einen eindeutigen Zustand, den alle Clients und der Server gleich berechnen.

**Entscheidung:**

1. Jede Einheit hat einen **Kampfzustand** (`CombatState`): `OutOfCombat`, `InCombat { since, last_hostile_tick }` oder `Dead`. Er ist Simulationszustand, Teil von Snapshot und Beobachtung.
2. **In den Kampf** kommt eine Einheit im Tick, in dem sie einer feindlichen Einheit Schaden zufügt, von ihr Schaden nimmt oder einen Zauber auf sie wirkt. Ein Pet zieht seinen Besitzer mit in den Kampf, ein Reittier seinen Reiter, und umgekehrt.
3. **Aus dem Kampf** kommt sie, wenn seit der letzten feindlichen Handlung eine feste Zahl Ticks vergangen ist (Platzhalter 300 = 5 s) und keine feindliche Einheit sie mehr als Ziel führt.
4. **Tot** ist sie vom Tod bis zum Respawn; danach ist sie außer Kampf.
5. **Charakter ändern** (Fähigkeitenleiste, Ausrüstung, später mehr): erlaubt außer Kampf und im Tod (wirkt ab dem Respawn), verboten im Kampf. Eine Änderung ist ein eigener Befehl im Eingabestrom, den die Simulation im Set Intent prüft und anwendet oder ablehnt; im Netzwerk prüft der Server.
6. Der Zustand wird im Set Schaden (Eintritt) und im Set Aufräumen (Austritt, Tod) fortgeschrieben, in Ticks, nie nach Uhrzeit.

**Verworfene Alternative:** Änderungen nur an festen Orten oder nur zwischen Runden. Verworfen als allgemeine Regel, weil die Welt keine Runden hat; Orte können ein Spielmodus zusätzlich verlangen.

**Konsequenzen:** Kampfzustand wird an mehreren Stellen gebraucht (HUD, KI, Regeneration außer Kampf, Aufsitzen nur außer Kampf als Vorschlag). P-020 ist damit zur Hälfte beantwortet (wann), offen bleibt, was erlaubt ist. Details: [[einheiten|Einheiten]], [[faehigkeiten|Fähigkeiten & Belegung]].
