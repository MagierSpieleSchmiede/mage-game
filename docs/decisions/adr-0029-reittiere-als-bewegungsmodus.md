---
title: "ADR-0029 — Reittiere sind Kreaturen mit eigenem Leben, am Boden oder fliegend; der Reiter steuert und kämpft im Sattel"
---
# ADR-0029 — Reittiere sind Kreaturen mit eigenem Leben, am Boden oder fliegend; der Reiter steuert und kämpft im Sattel

**Status:** vorgeschlagen (überarbeitet 2026-11-15: Reittiere haben eigenes Leben, Daniels Vorgabe; damit ist [[p-019-reittier-als-eigenes-ziel|P-019]] beantwortet)

**Kontext:** Es soll Reittiere geben: am Boden und fliegend. Auf beiden soll gekämpft werden können, und Reittiere sollen eigenes Leben haben, also selbst angegriffen werden können. Die Simulation bewegt Einheiten kinematisch mit `MoveAndSlide` ([[adr-0003-kinematik-statt-dynamischer-rigidbodies|ADR-0003]]); Kreaturen sind Einheiten wie Magier ([[adr-0031-einheiten-magier-und-kreaturen|ADR-0031]]).

**Entscheidung:**

1. Ein Reittier ist eine **Kreatur** mit der Component `Rideable`: eigenes Leben, eigene Attribute, eigener Kollider, Bewegungsart `Ground` oder `Flying` aus dem `CreatureBlueprint`.
2. **Aufsitzen** bindet den Magier an das Reittier (`Rider` am Reittier, `Riding` am Magier). Solange er aufsitzt, steuert sein Intent das Reittier; der Magier bewegt sich mit, sein eigener `MoveAndSlide` ruht.
3. **Fliegen** ist dreidimensionale Bewegung mit `MoveAndSlide`: Der Intent bekommt eine vertikale Achse (steigen, sinken); am Boden wird sie ignoriert.
4. **Kampf im Sattel:** Der Reiter zaubert wie zu Fuß, außer ein Blueprint verlangt „nur abgesessen“. Treffer treffen, wen der Shape-Cast zuerst berührt, Reittier oder Reiter. Stirbt das Reittier, wird der Reiter abgeworfen; in der Luft sinkt er zu Boden.
5. **Aufsitzen** ist eine Fähigkeit (Slot, Wirkzeit in Ticks), nur außer Kampf ([[adr-0032-kampfzustand|ADR-0032]]). Ein eigenes Reittier ist ein Pet des Magiers und kehrt nach seinem Tod mit Abklingzeit zurück.

**Verworfene Alternative:** Reittier als bloßer Zustand des Magiers (die erste Fassung dieser ADR). Verworfen, weil ein Reittier dann kein eigenes Leben haben und nicht gezielt angegriffen werden kann.

**Konsequenzen:** Zwei Entities bewegen sich gekoppelt; Snapshot und Rollback müssen die Bindung (`Rider`/`Riding`) mitnehmen, und die Bewegung des Reiters folgt im selben Tick der des Reittiers. `MageIntent` bekommt eine vertikale Achse; Bodenzauber (`GroundArea`) aus der Luft zielen weiter auf den Boden. Details: [[reittiere|Reittiere]], [[einheiten|Einheiten]].
