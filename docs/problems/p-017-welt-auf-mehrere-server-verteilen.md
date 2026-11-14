---
title: "P-017 — Die große Welt auf mehrere Serverprozesse verteilen"
---
# P-017 — Die große Welt auf mehrere Serverprozesse verteilen

Ein Serverprozess, der tausende Spieler mit allen Zaubern bei 60 Ticks pro Sekunde simuliert, wird das Budget sprengen ([[benchmarks|Benchmarks]]). Möglichkeiten: feste Gebiete pro Prozess mit Übergabe an Grenzen, dynamische Aufteilung nach Spielerdichte, Kopien voller Gebiete (Layer) mit Obergrenze pro Kopie. Jede braucht eine Regel für Kampf über eine Gebietsgrenze und für die `SimId`-Vergabe über Prozesse hinweg ([[welt|Welt]], [[adr-0028-grosse-welt-mit-interessenbereich|ADR-0028]]).

Nächster Schritt: mit dem Netzwerk-Milestone (M7) einen Lasttest mit simulierten Clients bauen; davor messen, wie viele Magier ein Prozess schafft (Benchmark `world_crowd`).
