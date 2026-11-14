---
title: "Eingabegeräte — Tastatur und Maus, Controller, Touch"
---
# Eingabegeräte — Tastatur und Maus, Controller, Touch

Gespielt wird mit Tastatur und Maus oder mit Controller, auf Mobilgeräten per Touch. Alle Geräte erzeugen **Aktionen**, aus denen der `HumanController` den `MageIntent` baut ([[adr-0022-eingabe-ueber-aktionen|ADR-0022]], vorgeschlagen). Die Simulation weiß nicht, welches Gerät benutzt wird ([[intent-steuerung|Intent & Steuerung]]).

## Standardbelegung

| Aktion | Tastatur und Maus | Controller | Touch (Mobile) |
| --- | --- | --- | --- |
| Bewegen | `W` `A` `S` `D` | linker Stick | virtueller Stick links |
| Kamera und Zielen | Maus | rechter Stick | Wischen rechts |
| Zauber 1 (Bolt) | `1` oder linke Maustaste | rechter Trigger | Knopf |
| Zauber 2 (Homing-Bolt) | `2` | rechte Schultertaste | Knopf |
| Zauber 3 (Feuerball) | `3` | linker Trigger | Knopf |
| Menü | `Esc` | Start | Menü-Knopf |

Platzhalter, wird im Spieltest angepasst. Jede Belegung ist vom Spieler änderbar und wird gespeichert.

## Regeln

- **Gehalten, gedrückt, losgelassen** gelten für alle Geräte gleich; das Flanken-Protokoll im `InputBuffer` bleibt unverändert.
- **Zielrichtung** entsteht immer aus der Kamera (Strahl durch das Fadenkreuz). Mit Controller dreht der rechte Stick die Kamera; Empfindlichkeit, Totzone und Beschleunigung sind Einstellungen.
- **Gerätewechsel** während des Spiels ist erlaubt; das HUD zeigt die Symbole des zuletzt benutzten Geräts.
- **Mehrere lokale Spieler** (Splitscreen) sind nicht geplant, aber die Zuordnung Gerät → `PlayerId` wird nicht fest verdrahtet.
- Texte und Tastensymbole im HUD kommen aus der [[lokalisierung|Lokalisierung]].

## Was offen ist

Eingabe-Crate und Zielhilfe für Controller: [[p-011-eingabe-crate-und-zielhilfe|P-011]].
