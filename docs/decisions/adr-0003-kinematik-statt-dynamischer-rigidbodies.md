---
title: "ADR-0003 — Kinematik statt dynamischer RigidBodies: Avian nur für statische Kollider und Queries"
---
# ADR-0003 — Kinematik statt dynamischer RigidBodies: Avian nur für statische Kollider und Queries

**Status:** angenommen

**Kontext:** Rollback braucht einen snapshotbaren Zustand; Physik-Interna sind schwer zu serialisieren.

**Entscheidung:** Magier sind kinematische Character Controller, Projektile werden von der Simulation bewegt und per Shape-Cast geprüft. Avian liefert die statischen Arena-Kollider, die kinematischen Magier-Kollider, `SpatialQuery` und `MoveAndSlide`, sonst nichts. Dynamische RigidBodies für Magier oder Projektile sind untersagt.

**Verworfene Alternative:** Projektile und Magier als dynamische Körper mit Kollisions-Events. Verworfen, weil Tunnelling dann vom Solver-Timestep abhinge und die Rollback-Schicht den Physikzustand mitfassen müsste.

**Konsequenzen:** Explizite Shape-Casts sind mehr Code; im Gegenzug liegt der gesamte simulationsrelevante Zustand in eigenen Components.
