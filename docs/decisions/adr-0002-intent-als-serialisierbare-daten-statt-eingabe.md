---
title: "ADR-0002 — Intent als serialisierbare Daten statt Eingabe-Lesen in der Simulation"
---
# ADR-0002 — Intent als serialisierbare Daten statt Eingabe-Lesen in der Simulation

**Status:** angenommen

**Kontext:** Menschen, KI und Netzwerk sollen dieselben Regeln durchlaufen.

**Entscheidung:** Die einzige Schnittstelle zur Simulation ist `MageIntent`; Simulations-Systeme lesen keine Eingabegeräte.

**Verworfene Alternative:** Simulations-Systeme mit `Res<ButtonInput>`. Verworfen, weil das KI und Netcode unmöglich machte und Tests ein Fenster brauchten.

**Konsequenzen:** Der Dummy ist ein vollwertiger Magier mit leerem Intent; ein KI-Controller ist ein weiterer Intent-Produzent.
