---
title: "Fähigkeiten & Belegung — der Spieler stellt Leiste und Tasten selbst zusammen"
---
# Fähigkeiten & Belegung — der Spieler stellt Leiste und Tasten selbst zusammen

Jeder Spieler wählt seine Fähigkeiten und legt fest, auf welcher Taste welche liegt. Grundsatz: [[adr-0030-faehigkeitenleiste-vom-spieler|ADR-0030]].

## Zwei Teile, zwei Orte

| Teil | Inhalt | gehört zu | gespeichert |
| --- | --- | --- | --- |
| Fähigkeitenleiste (`Loadout`) | Slot 1 bis `MAX_SLOTS` → `SpellId` oder andere Fähigkeit (Aufsitzen) | Simulation: Component am Magier, Teil von Szenario, Snapshot und Beobachtung | Spieler-Profil, vom Server geprüft |
| Belegung | Aktion „Slot n“ → Taste, Maustaste, Controller-Knopf, Touch-Knopf | Präsentation ([[eingabegeraete\|Eingabegeräte]]) | Einstellungen auf dem Gerät |

Beispiel einer deklarierten Leiste (Format offen, Vorschlag RON):

```ron
Loadout(slots: [Some("bolt"), Some("homing_bolt"), Some("fireball"), None, Some("mount_griffin")])
```

## Regeln

- Der Intent kennt nur Slot-Nummern; welche Fähigkeit in einem Slot liegt, steht in der Leiste des Magiers. Wechselt der Spieler seine Belegung, ändert sich für die Simulation nichts.
- **Ändern** darf der Spieler Leiste und Ausrüstung nur außer Kampf oder im Tod ([[adr-0032-kampfzustand|ADR-0032]]); die Änderung ist ein Befehl im Eingabestrom, den die Simulation prüft.
- Die Leiste wird bei Rundenbeginn, beim Betreten der Welt und bei jeder Änderung geprüft: nur bekannte und erlaubte Fähigkeiten, höchstens `MAX_SLOTS`, keine doppelt. Ungültig heißt: abgelehnt, nicht stillschweigend korrigiert.
- Namen und Beschreibungen der Fähigkeiten kommen aus der [[lokalisierung|Lokalisierung]]; in der Leiste stehen nur IDs.
- KI, Add-ons und Kreaturen ([[einheiten|Einheiten]]) nutzen dieselben Leisten; RL kann Leisten als Teil des Balancings variieren ([[ki-und-rl|KI & RL]]).

## Was offen ist

- [[p-020-faehigkeitenleiste-aendern|P-020]]: Wer legt fest, welche Fähigkeiten und welche Ausrüstung erlaubt sind?
