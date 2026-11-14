---
title: "ADR-0007 — Shape-Cast pro Tick für Projektile statt Kollisions-Events"
---
# ADR-0007 — Shape-Cast pro Tick für Projektile statt Kollisions-Events

**Kontext:** Projektile dürfen dünne Hindernisse nicht durchtunneln.
**Entscheidung:** Die Bewegung zeichnet pro Tick ein Travel-Segment; das Set Treffer castet einen Shape entlang des gesamten Segments.
**Verworfene Alternative:** Physik-Kontakte des Solvers. Verworfen, weil Kontaktzustand solverabhängig und schlecht snapshotbar ist ([[adr-0003-kinematik-statt-dynamischer-rigidbodies|ADR-0003]]).
**Konsequenzen:** Treffer und Bewegung sind sauber getrennte Sets; schnelle Projektile kosten einen Cast pro Tick.
