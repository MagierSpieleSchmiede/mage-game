---
title: "ADR-0009 — Eine geseedete RNG-Ressource statt verstreuter Zufallsquellen"
---
# ADR-0009 — Eine geseedete RNG-Ressource statt verstreuter Zufallsquellen

**Status:** angenommen

**Kontext:** Determinismustest und spätere Netcode-Desync-Analyse brauchen einen einzigen Zufallsstrom.

**Entscheidung:** Eine RNG-Ressource (seedbar, serialisierbarer Zustand) ist die einzige Zufallsquelle der Simulation.

**Verworfene Alternative:** `rand::thread_rng` oder mehrere lokale Generatoren. Verworfen, weil unsortierte Ströme nicht reproduzierbar und nicht snapshotbar sind.

**Konsequenzen:** Der RNG-Zustand gehört in jeden Snapshot; Systeme müssen den Strom in definierter Reihenfolge ziehen (Invariante 9).
