---
title: "P-018 — Sichtbarkeit pro Team skaliert nicht auf eine offene Welt"
---
# P-018 — Sichtbarkeit pro Team skaliert nicht auf eine offene Welt

`VisibleTo` ist eine Bitmaske mit einem Bit pro Team, höchstens 32 Teams ([[adr-0018-sichtbarkeit-als-simulationszustand|ADR-0018]]). In einer offenen Welt mit tausenden Spielern gibt es viele Gruppen, die sich ständig neu bilden, und Sichtbarkeit wird zusätzlich durch den Interessenbereich begrenzt ([[welt|Welt]]). Möglichkeiten: Sichtbarkeit pro Zelle nur für die Teams berechnen, die dort anwesend sind (Bitmaske lokal pro Zelle), oder Sichtbarkeit pro Beobachter statt pro Team.

Nächster Schritt: vor M5 (Unsichtbarkeit) entscheiden, als Änderung an ADR-0018, solange sie vorgeschlagen ist. Die Arena bleibt bei der Bitmaske.
