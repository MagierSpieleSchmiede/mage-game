---
title: "P-015 — Trefferwertung unter Latenz"
---
# P-015 — Trefferwertung unter Latenz

Mit Server-Autorität sieht ein Spieler fremde Magier leicht in der Vergangenheit (Interpolation) und seinen eigenen in der Zukunft (Vorhersage). Ein Projektil, das auf dem eigenen Bildschirm trifft, kann auf dem Server danebengehen. Optionen: keine Kompensation (Projektile haben Flugzeit, das mildert es), Rückrechnung der Zielpositionen beim Abschuss auf dem Server (begünstigt den Schützen, benachteiligt den Getroffenen), oder ein Mittelweg mit Obergrenze.

Nächster Schritt: in M7 mit den Latenztests ([[synchronisation-latenz|Synchronisation & Latenz]]) messen, wie oft sichtbare Treffer bei 50, 100 und 200 ms verloren gehen, und dann als ADR entscheiden.
