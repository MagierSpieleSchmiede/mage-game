---
title: "P-005 — Avians Physik in FixedUpdate einhängen und ordnen"
---
# P-005 — Avians Physik in FixedUpdate einhängen und ordnen

Die Kette verlangt, dass Avians Schritt zwischen Bewegung und Treffer liegt. Avian lief bisher standardmäßig in einem eigenen Fixed-Schedule nach `FixedUpdate`; dann würden die Shape-Casts gegen Magier-Positionen des Vorticks prüfen. Nächster Schritt: in M1 gegen die Doku von Avian 0.8 prüfen, die Physik ausdrücklich in `FixedUpdate` einhängen, als Set Physik in die Kette ordnen und mit einem Integrationstest festschreiben (ein Projektil trifft einen Magier, der sich im selben Tick in die Flugbahn bewegt hat).
