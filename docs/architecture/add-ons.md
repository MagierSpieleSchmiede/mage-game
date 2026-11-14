---
title: "Add-ons — den eigenen Charakter auslesen und automatisieren"
---
# Add-ons — den eigenen Charakter auslesen und automatisieren

Spieler sollen Add-ons nutzen können, die ihren Charakter **auslesen** (eigene Anzeigen, Statistiken, Aufzeichnung) oder seine Aktionen **automatisieren** (Abläufe, Makros, programmierte Magier). Grundsatz: [[adr-0025-add-ons-als-intent-produzenten|ADR-0025]]. Gebaut wird es in Milestone 8.

## Wie ein Add-on arbeitet

1. Es ist ein WebAssembly-Modul mit einer kleinen, versionierten Schnittstelle.
2. Pro Tick (oder alle k Ticks) bekommt es die [[beobachtung|Beobachtung]] seines Spielers, nichts sonst.
3. Es darf zurückgeben: einen vorgeschlagenen `MageIntent` (Automatisierung) und, später, Anzeige-Elemente fürs HUD (Auslesen).
4. Der Intent geht in den `InputBuffer` wie menschliche Eingabe. Was der Spieler gleichzeitig drückt, hat Vorrang (Regel je Aktion einstellbar).
5. Im Netzwerk prüft der Server Add-on-Intents genau wie alle anderen ([[sicherheit|Sicherheit]]).

## Sandbox

| Erlaubt | Nicht erlaubt |
| --- | --- |
| Beobachtung lesen, Intent schreiben, eigenen Speicher im Modul | Dateisystem, Netzwerk, Uhr (außer Tick-Nummer), andere Spieler, ECS-Welt |
| begrenzte Rechenzeit pro Tick (Überschreitung: Intent verfällt, wiederholt: Add-on wird angehalten) | Threads, unbegrenzter Speicher |

## Spielmodus-Regeln

Jede Runde legt fest: **keine Add-ons**, **nur Auslesen** oder **auch Automatisieren**. Der Server kann Automatisierung nicht sicher erkennen; die Regel ist deshalb eine Abmachung, die der offizielle Client durchsetzt. Details und Fairness: [[p-010-add-on-laufzeit-und-fairness|P-010]].

## Verwandt

- KI-Gegner und Add-ons nutzen dieselbe Schnittstelle; ein trainierter Agent kann als Add-on verpackt werden.
- Ein „programmierter Magier“-Spielmodus (Spieler schreiben Add-ons, die gegeneinander antreten) wäre damit ohne neue Simulationstechnik möglich.
