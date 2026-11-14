---
title: "ADR-0013 — SimId statt Entity-Index für jede ergebnisrelevante Reihenfolge"
---
# ADR-0013 — SimId statt Entity-Index für jede ergebnisrelevante Reihenfolge

**Kontext:** Wo mehrere Entities im selben Tick interagieren, entscheidet die Reihenfolge über das Ergebnis. Entity-Indizes sind nach einem Rollback-Restore andere als vorher.
**Entscheidung:** Jede Simulations-Entity bekommt beim Spawn eine `SimId` aus einem fortlaufenden Zähler; Zähler und IDs liegen im Snapshot. Sortiert wird ausschließlich nach `SimId`.
**Verworfene Alternative:** Sortierung nach Entity-Index plus Snapshot der Spawn-Reihenfolge. Verworfen, weil der Restore dann die Indizes exakt reproduzieren müsste, was von Bevy-Interna abhängt.
**Konsequenzen:** Vier Bytes pro Entity; der Restore darf Entities beliebig neu anlegen.
