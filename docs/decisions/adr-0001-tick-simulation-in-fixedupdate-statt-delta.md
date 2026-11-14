---
title: "ADR-0001 — Tick-Simulation in FixedUpdate statt Delta-Zeit in Update"
---
# ADR-0001 — Tick-Simulation in FixedUpdate statt Delta-Zeit in Update

**Kontext:** Rollback-Netcode und KI brauchen wiederholbare, vergleichbare Simulationsschritte.
**Entscheidung:** Der komplette Gameplay-Zustand schreitet nur im `FixedUpdate` in einer geordneten `SystemSet`-Kette fort; alle Timer sind Tick-Zähler.
**Verworfene Alternative:** Systeme in `Update` mit Delta-Zeit. Verworfen, weil Anzahl und Inhalt der Schritte dann von der Framerate abhingen und Snapshots nicht vergleichbar wären.
**Konsequenzen:** Präsentation muss interpolieren; jede neue Mechanik muss einem Set zugeordnet werden.
