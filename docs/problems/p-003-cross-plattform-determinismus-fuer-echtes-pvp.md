---
title: "P-003 — Cross-Plattform-Determinismus für echtes PvP"
---
# P-003 — Cross-Plattform-Determinismus für echtes PvP

`f32` und Solver-Queries sind über Architekturen hinweg nicht bit-garantierbar. Für Netzwerk-PvP später: Plattformen locken oder Fixed-Point-Betrachtung. Der Prototyp trifft die Entscheidung bewusst nicht vor und garantiert nur dieselbe Plattform, derselbe Build.

**Stand nach [[adr-0017-server-autoritativ-mit-client-vorhersage|ADR-0017]] (vorgeschlagen):** Mit einem Server, der die einzige maßgebliche Simulation führt, führen plattformübergreifende Abweichungen nur noch zu kleinen Korrekturen bei der Client-Vorhersage, nicht zu Desyncs. Bit-Gleichheit bleibt wünschenswert für Replays über Plattformgrenzen; dafür wäre Avians Feature `enhanced-determinism` zu prüfen. Nächster Schritt: in M7 messen, wie oft Vorhersagen zwischen Plattformen abweichen.
