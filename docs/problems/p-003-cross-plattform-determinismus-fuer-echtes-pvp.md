---
title: "P-003 — Cross-Plattform-Determinismus für echtes PvP"
---
# P-003 — Cross-Plattform-Determinismus für echtes PvP

`f32` und Solver-Queries sind über Architekturen hinweg nicht bit-garantierbar. Für Netzwerk-PvP später: Plattformen locken oder Fixed-Point-Betrachtung. Der Prototyp trifft die Entscheidung bewusst nicht vor und garantiert nur dieselbe Plattform, derselbe Build.

**Stand nach [[adr-0017-rollback-mit-server-als-schiedsrichter|ADR-0017]] (angenommen):** Der Server ist Schiedsrichter; plattformübergreifende Abweichungen im Rollback der Clients führen nur zu Korrekturen durch Server-Zustand, nicht zu dauerhaften Desyncs. Je seltener sie sind, desto besser bleibt das Rollback-Gefühl. Bit-Gleichheit bleibt wünschenswert für Replays über Plattformgrenzen; dafür wäre Avians Feature `enhanced-determinism` zu prüfen. Nächster Schritt: in M7 messen, wie oft Vorhersagen zwischen Plattformen abweichen.
