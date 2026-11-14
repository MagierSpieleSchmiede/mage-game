---
title: "Eingabegeräte — Tastatur und Maus, Controller, Touch"
---
# Eingabegeräte — Tastatur und Maus, Controller, Touch

Gespielt wird mit Tastatur und Maus oder mit Controller, auf Mobilgeräten per Touch. Alle Geräte erzeugen **Aktionen**, aus denen der `HumanController` den `MageIntent` baut ([[adr-0022-eingabe-ueber-aktionen|ADR-0022]]). Die Simulation weiß nicht, welches Gerät benutzt wird ([[intent-steuerung|Intent & Steuerung]]).

## Standardbelegung

| Aktion | Tastatur und Maus | Controller | Touch (Mobile) |
| --- | --- | --- | --- |
| Bewegen | `W` `A` `S` `D` | linker Stick | virtueller Stick links |
| Kamera und Zielen | Maus | rechter Stick | Wischen rechts |
| Slot 1 (Standard: Bolt) | `1` oder linke Maustaste | rechter Trigger | Knopf |
| Slot 2 (Standard: Homing-Bolt) | `2` | rechte Schultertaste | Knopf |
| Slot 3 (Standard: Feuerball) | `3` | linker Trigger | Knopf |
| Slot 4 bis `MAX_SLOTS` | `4` … `0`, `Q`, `E` | Schultertaste + Knopf | Knopf |
| Menü | `Esc` | Start | Menü-Knopf |

Platzhalter, wird im Spieltest angepasst. Welche Fähigkeit in einem Slot liegt, legt der Spieler in seiner Fähigkeitenleiste fest ([[faehigkeiten|Fähigkeiten & Belegung]]). Jede Belegung ist vom Spieler änderbar und wird gespeichert. Das README zeigt eine Kopie der Spalten Tastatur und Controller; wer hier ändert, zieht sie mit.

## Regeln

- **Gehalten, gedrückt, losgelassen** gelten für alle Geräte gleich; das Flanken-Protokoll im `InputBuffer` bleibt unverändert.
- **Zielrichtung** entsteht immer aus der Kamera (Strahl durch das Fadenkreuz). Mit Controller dreht der rechte Stick die Kamera; Empfindlichkeit, Totzone und Beschleunigung sind Einstellungen.
- **Gerätewechsel** während des Spiels ist erlaubt; das HUD zeigt die Symbole des zuletzt benutzten Geräts.
- **Mehrere lokale Spieler** (Splitscreen) sind nicht geplant, aber die Zuordnung Gerät → `PlayerId` wird nicht fest verdrahtet.
- Texte und Tastensymbole im HUD kommen aus der [[lokalisierung|Lokalisierung]].

## Was offen ist

Eingabe-Crate und Zielhilfe für Controller: [[p-011-eingabe-crate-und-zielhilfe|P-011]].
