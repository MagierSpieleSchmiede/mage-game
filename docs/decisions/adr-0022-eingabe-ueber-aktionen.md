---
title: "ADR-0022 — Eingabe über Aktionen statt über Tasten (Tastatur und Maus, Controller, Touch)"
---
# ADR-0022 — Eingabe über Aktionen statt über Tasten (Tastatur und Maus, Controller, Touch)

**Status:** vorgeschlagen

**Kontext:** Das Spiel soll mit Tastatur und Maus und mit Controller spielbar sein, auf Mobilgeräten auch per Touch. Tasten fest im Code abzufragen würde jedes Gerät zu einem Sonderfall machen.

**Entscheidung:** Der `HumanController` liest **Aktionen** (`Bewegen`, `Kamera`, `Zauber1` … `ZauberN`, später weitere), nicht Tasten. Eine Belegungstabelle pro Gerät ordnet Tasten, Sticks, Trigger und Touch-Flächen den Aktionen zu; die Belegung ist vom Spieler änderbar und wird gespeichert. Aus den Aktionen entsteht wie bisher der `MageIntent`; die Simulation erfährt nie, welches Gerät benutzt wurde.

**Verworfene Alternative:** Direktes Abfragen von Tasten und Gamepad-Knöpfen im Controller. Verworfen, weil Umbelegen, Controller und Touch dann jeweils eigene Codepfade bräuchten.

**Konsequenzen:** Die Zielrichtung entsteht bei allen Geräten aus der Kamera; mit Controller dreht der rechte Stick die Kamera. Ob es eine Zielhilfe für Controller gibt, ist eine Fairness-Frage ([[p-011-eingabe-crate-und-zielhilfe|P-011]]). Details: [[eingabegeraete|Eingabegeräte]].
