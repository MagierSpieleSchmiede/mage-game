---
title: "Reittiere — Kreaturen mit eigenem Leben, am Boden oder fliegend, Kampf im Sattel"
---
# Reittiere — Kreaturen mit eigenem Leben, am Boden oder fliegend, Kampf im Sattel

Magier können auf Reittieren reiten oder fliegen und dabei kämpfen; Reittiere haben eigenes Leben. Grundsatz: [[adr-0029-reittiere-als-bewegungsmodus|ADR-0029]]. Ein Reittier ist eine Kreatur ([[einheiten|Einheiten]]).

## Ablauf

| Schritt | Was passiert |
| --- | --- |
| Aufsitzen | Fähigkeit in einem Slot der [[faehigkeiten\|Fähigkeitenleiste]], nur außer Kampf; Wirkzeit in Ticks (Platzhalter 60), unterbrochen durch Schaden |
| Aufgesessen | `Rider(SimId)` am Reittier, `Riding(SimId)` am Magier; der Intent des Magiers steuert das Reittier, der Magier bewegt sich mit |
| Fliegen | `MoveAndSlide` in drei Dimensionen; vertikale Achse aus dem Intent; Mindest- und Höchsthöhe aus der Welt |
| Absitzen | Fähigkeit, oder erzwungen: Tod des Reiters, Tod des Reittiers (Abwurf, im Flug Sinkflug), Zone ohne Reittiere |

## Kampf

- Der Reiter zaubert wie zu Fuß, außer ein Blueprint verlangt „nur abgesessen“.
- Treffer treffen, wen der Shape-Cast zuerst berührt: Reittier oder Reiter. Flächenschaden trifft beide.
- `GroundArea` aus der Luft: Zielpunkt bleibt der Schnitt des Zielstrahls mit Boden oder Hindernis.
- Reittier und Reiter teilen den Kampfzustand ([[adr-0032-kampfzustand|ADR-0032]]).

## Was offen ist

- Ob mehrere Reiter auf einem Reittier sitzen können (`Rideable { seats }`): Spieldesign, nach M10.
- Animation und Modelle: Präsentation, nach der Asset-Pipeline ([[adr-0021-animation-ist-reine-praesentation|ADR-0021]]).
