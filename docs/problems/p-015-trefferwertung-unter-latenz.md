---
title: "P-015 — Trefferwertung unter Latenz"
---
# P-015 — Trefferwertung unter Latenz

Mit Rollback sieht ein Spieler fremde Magier dort, wo sie wären, wenn sie ihre letzte Eingabe beibehalten haben. Ändert ein Gegner seine Richtung, korrigiert der Rollback erst eine Paketlaufzeit später; ein Projektil, das auf dem eigenen Bildschirm traf, kann dann im maßgeblichen Verlauf danebengehen, und umgekehrt (Treffer, die man nicht kommen sah). Optionen: keine Kompensation (Projektile haben Flugzeit, das mildert es), Rückrechnung der Zielpositionen beim Abschuss auf dem Server (begünstigt den Schützen, benachteiligt den Getroffenen), oder ein Mittelweg mit Obergrenze.

Nächster Schritt: in M7 mit den Latenztests ([[synchronisation-latenz|Synchronisation & Latenz]]) messen, wie oft sichtbare Treffer bei 50, 100 und 200 ms verloren gehen, und dann als ADR entscheiden.
