---
title: "Reittiere — Bodenreittiere und Flugreittiere, Kampf im Sattel"
---
# Reittiere — Bodenreittiere und Flugreittiere, Kampf im Sattel

Magier können auf Reittieren reiten oder fliegen und dabei kämpfen. Grundsatz: [[adr-0029-reittiere-als-bewegungsmodus|ADR-0029]].

## Ablauf

| Schritt | Was passiert |
| --- | --- |
| Aufsitzen | Fähigkeit in einem Slot der [[faehigkeiten\|Fähigkeitenleiste]]; Wirkzeit in Ticks (Platzhalter 60), unterbrochen durch Schaden |
| Aufgesessen | Component `Mounted { mount }`; Bewegungsart, Kollider und Attribut-Modifikatoren aus dem `MountBlueprint` |
| Fliegen | `MoveAndSlide` in drei Dimensionen; vertikale Achse aus dem Intent; Mindest- und Höchsthöhe aus der Welt |
| Absitzen | Fähigkeit oder erzwungen (Tod, Betreten einer Zone ohne Reittiere); in der Luft: Sinkflug bis zum Boden, kein Fallschaden im Prototyp |

## Kampf

- Zauber wirken im Sattel wie zu Fuß, außer ein Blueprint verlangt „nur abgesessen“.
- Treffer treffen den Magier; der Kollider im Sattel ist größer (Platzhalter: Reittier-Form).
- `GroundArea` aus der Luft: Zielpunkt bleibt der Schnitt des Zielstrahls mit Boden oder Hindernis.
- Homing und Sichtlinien sind schon dreidimensional; Flughöhe ändert nur, was sichtbar ist.

## Was offen ist

- [[p-019-reittier-als-eigenes-ziel|P-019]]: Haben Reittiere eigenes Leben, und kann man abgeworfen werden?
- Animation und Modelle der Reittiere: Präsentation, nach der Asset-Pipeline ([[adr-0021-animation-ist-reine-praesentation|ADR-0021]]).
