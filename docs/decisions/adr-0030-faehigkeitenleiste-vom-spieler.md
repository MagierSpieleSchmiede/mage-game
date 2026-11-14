---
title: "ADR-0030 — Der Spieler stellt seine Fähigkeitenleiste und ihre Tastenbelegung selbst zusammen"
---
# ADR-0030 — Der Spieler stellt seine Fähigkeitenleiste und ihre Tastenbelegung selbst zusammen

**Status:** angenommen (2026-11-15)

**Kontext:** Spieler sollen selbst festlegen, welche Fähigkeiten sie haben und auf welchen Tasten sie liegen. Bisher waren es genau drei feste Slots (Bolt, Homing-Bolt, Feuerball). Eingabe läuft schon über Aktionen mit änderbarer Belegung ([[adr-0022-eingabe-ueber-aktionen|ADR-0022]]).

**Entscheidung:**

1. Jeder Spieler hat eine **Fähigkeitenleiste** (`Loadout`): eine geordnete Liste von bis zu `MAX_SLOTS` Einträgen (Platzhalter 12), jeder Eintrag eine `SpellId` (oder eine andere Fähigkeit wie Aufsitzen). Sie ist Simulationszustand des Magiers: Der Intent sagt nur „Slot 4 gedrückt“, die Leiste sagt, welche Fähigkeit das ist.
2. Der Spieler **deklariert** seine Leiste als Daten (eine Datei im Spieler-Profil, im Spiel bearbeitbar), ebenso die Belegung jeder Slot-Aktion auf Tastatur, Maus, Controller oder Touch. Leiste und Belegung sind getrennt: Die Leiste gehört zur Simulation, die Belegung zur Präsentation.
3. Die Simulation **prüft** jede Leiste: nur erlaubte Fähigkeiten, höchstens `MAX_SLOTS`, keine Fähigkeit doppelt. Im Netzwerk prüft der Server; eine ungültige Leiste wird abgelehnt ([[sicherheit|Sicherheit]]).
4. Die Leiste wird beim Betreten einer Runde oder Welt übernommen; ob und wann sie sich während des Spiels ändern darf, ist offen ([[p-020-faehigkeitenleiste-aendern|P-020]]).

**Verworfene Alternative:** Feste Slots pro Klasse. Verworfen, weil gewünscht ist, dass Spieler ihre Fähigkeiten selbst wählen; Klassen können später als Einschränkung der erlaubten Fähigkeiten dazukommen.

**Konsequenzen:** `held` und `pressed` im Intent haben `MAX_SLOTS` Einträge statt drei. Das Casting-System liest die Fähigkeit aus der Leiste des Magiers statt aus einer festen Tabelle. Das Szenario-Format bekommt pro Spieler eine Leiste. Die Leiste ist Teil des Snapshots und der Beobachtung. Details: [[faehigkeiten|Fähigkeiten & Belegung]].
