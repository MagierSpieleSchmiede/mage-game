---
title: "ADR-0014 — Impacts sammeln, dann sortiert auslösen"
---
# ADR-0014 — Impacts sammeln, dann sortiert auslösen

**Kontext:** Ein Observer läuft dort, wo er ausgelöst wird. Würde das Set Treffer direkt auslösen, liefe der Schaden im falschen Set und in der Reihenfolge der Query-Iteration.
**Entscheidung:** Das Set Treffer schreibt Impacts nur in `PendingImpacts`. Das Set Schaden sortiert sie nach `SimId` des Zaubers, löst `SpellImpact` aus und leert den Puffer; die Folge-Trigger laufen in diesem Set durch.
**Verworfene Alternative:** Direktes Auslösen im Set Treffer. Verworfen, weil die Reihenfolge zweier Treffer im selben Tick dann undefiniert wäre, etwa wer wen zuerst tötet.
**Konsequenzen:** `PendingImpacts` ist am Tick-Ende immer leer und gehört in keinen Snapshot; durch Split erzeugte Fragmente existieren ab dem Folgetick.
