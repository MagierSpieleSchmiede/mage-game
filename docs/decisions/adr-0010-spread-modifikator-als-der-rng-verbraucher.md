---
title: "ADR-0010 — Spread-Modifikator als der RNG-Verbraucher des Prototyps"
---
# ADR-0010 — Spread-Modifikator als der RNG-Verbraucher des Prototyps

**Status:** angenommen

**Kontext:** Ohne Zufallsverbrauch wäre der Determinismustest leer.

**Entscheidung:** Der Modifikator `Spread` (on cast) streut Startrichtung beziehungsweise Zielpunkt mit der geseedeten RNG; Bolt und Feuerball tragen ihn.

**Verworfene Alternative:** Kein Zufall im Prototyp (Test wäre wertlos) oder Schadensvarianz (macht Treffer-Erwartungen in Tests unintuitiv).

**Konsequenzen:** Der Determinismustest läuft über den Bolt und prüft echte RNG-Teilnahme; `Spread` erfüllt nebenbei die Anforderung des geteilten Modifikators.
