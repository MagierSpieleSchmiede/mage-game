---
title: "ADR-0018 — Sichtbarkeit ist Simulationszustand, pro Team berechnet"
---
# ADR-0018 — Sichtbarkeit ist Simulationszustand, pro Team berechnet

**Status:** vorgeschlagen

**Kontext:** Unsichtbarkeit soll eine Spielmechanik werden. Wer was sieht, entscheidet über Zielwahl (Homing), über das, was ein Client empfangen darf, und über das, was KI und Add-ons erfahren. Würde nur die Präsentation Unsichtbares ausblenden, wüssten Simulation, KI und Netzwerk trotzdem alles.

**Entscheidung:** Ein Set **Sichtbarkeit** am Ende jedes Ticks berechnet für jede Simulations-Entity eine Bitmaske `VisibleTo` (ein Bit pro Team). Eigene Teammitglieder sind immer sichtbar. Ein gegnerischer Magier ist für ein Team sichtbar, wenn mindestens ein Mitglied ihn in Sichtweite und mit Sichtlinie hat und seine Tarnung die Wahrnehmung des Beobachters nicht übersteigt, oder wenn er gerade enthüllt ist (zum Beispiel durch Zaubern oder erlittenen Schaden). `VisibleTo` ist Teil des Snapshots. Homing, Beobachtung, Netzwerk-Versand und Darstellung lesen nur diese Maske.

**Verworfene Alternative:** Unsichtbarkeit als reiner Shader-Effekt in der Präsentation. Verworfen, weil dann jeder Client und jede KI die volle Information hätte.

**Konsequenzen:** Pro Tick fallen Sichtlinien-Raycasts an (Teams × Magier); das wird gemessen und mit räumlicher Vorauswahl begrenzt. Tarnung und Wahrnehmung sind Attribute ([[adr-0016-attribute-mit-modifikatoren|ADR-0016]]). Details: [[sichtbarkeit|Sichtbarkeit]].
