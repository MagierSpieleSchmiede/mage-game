---
title: "ADR-0008 — Ticks statt Sekunden für alle Timer"
---
# ADR-0008 — Ticks statt Sekunden für alle Timer

**Kontext:** Cooldowns, Fuse, Lebensdauer, Slow, Respawn müssen reproduzierbar ablaufen.
**Entscheidung:** Alle Dauerangaben sind `u32`-Tick-Zähler; die Tickrate ist eine Einstellung (Platzhalter 60 Hz) und wird ausdrücklich gesetzt, statt Bevys Standardwert zu übernehmen.
**Verworfene Alternative:** Sekunden mit Delta-Zeit akkumulieren. Verworfen wegen Drift und Nicht-Zählbarkeit.
**Konsequenzen:** Balance-Konstanten werden in Ticks ausgedrückt; die Balance-Ressource dokumentiert die Umrechnung.
