---
title: "ADR-0029 — Reittiere sind ein Bewegungsmodus des Magiers, Boden oder Flug, und Kampf geht auch im Sattel"
---
# ADR-0029 — Reittiere sind ein Bewegungsmodus des Magiers, Boden oder Flug, und Kampf geht auch im Sattel

**Status:** vorgeschlagen

**Kontext:** Es soll Reittiere geben: Reittiere am Boden und fliegende. Auf beiden soll gekämpft werden können. Die Simulation bewegt Magier kinematisch mit `MoveAndSlide` ([[adr-0003-kinematik-statt-dynamischer-rigidbodies|ADR-0003]]); Eigenschaften kommen aus Attributen ([[adr-0016-attribute-mit-modifikatoren|ADR-0016]]).

**Entscheidung:**

1. Ein Reittier ist ein **Blueprint** (`MountBlueprint`, Schlüssel `MountId`) wie ein Zauber: Bewegungsart (`Ground` oder `Flying`), Attribut-Modifikatoren (etwa Tempo +60 %), Kollider-Form im Sattel, Auf- und Absitzdauer in Ticks.
2. **Aufgesessen** ist ein Zustand des Magiers (Component `Mounted { mount: MountId }`), keine zweite gesteuerte Entity. Der Magier bleibt die eine Entity mit `PlayerId`, Leben und Mana; das Reittier ändert Bewegungsart, Kollider und Attribute.
3. **Fliegen** ist dreidimensionale Bewegung mit `MoveAndSlide`: Der Intent bekommt eine vertikale Achse (steigen, sinken); am Boden wird sie ignoriert. Keine Physik-Körper, kein Auftrieb, alles nach Ticks.
4. **Kampf im Sattel:** Zauber wirken aufgesessen wie zu Fuß. Ein Blueprint kann festlegen, dass er nur abgesessen geht. Treffer treffen den Magier; ob ein Reittier eigenes Leben hat und abgeworfen werden kann, ist offen ([[p-019-reittier-als-eigenes-ziel|P-019]]).
5. Aufsitzen ist eine Fähigkeit wie ein Zauber (Slot, Cooldown, Wirkzeit), also über die Fähigkeitenleiste belegbar ([[adr-0030-faehigkeitenleiste-vom-spieler|ADR-0030]]).

**Verworfene Alternative:** Reittier als eigene Entity, an die der Magier gekoppelt wird. Verworfen für den Anfang, weil dann zwei Entities synchron bewegt, gesnapshottet und getroffen werden müssten; das bleibt möglich, falls Reittiere eigenes Leben bekommen (P-019).

**Konsequenzen:** `MageIntent` bekommt eine vertikale Achse; Bodenzauber (`GroundArea`) brauchen eine Regel für Zaubernde in der Luft (Zielpunkt bleibt am Boden). Flug macht Sichtlinien und Interessenbereich dreidimensional; die Zellen der [[welt|Welt]] bleiben zweidimensional, Höhe zählt nur für Sicht und Treffer. Details: [[reittiere|Reittiere]].
