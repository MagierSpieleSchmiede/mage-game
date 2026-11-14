---
title: "P-023 — Phasen, Trefferzonen und besondere Fähigkeiten von Bossen als Daten"
---
# P-023 — Phasen, Trefferzonen und besondere Fähigkeiten von Bossen als Daten

Bosse wechseln Phasen (etwa bei 70 % und 30 % Leben), haben mehrere Trefferzonen mit eigenem Schadensfaktor, Fähigkeiten mit langer, angekündigter Wirkzeit und oft Bedrohungslisten (wen sie angreifen). Alles soll Daten im `CreatureBlueprint` sein, nicht Code pro Boss, und muss deterministisch nach Ticks laufen ([[einheiten|Einheiten]]). Offen: wie viel Ausdruckskraft die Daten brauchen (einfache Bedingungen oder eine kleine Skriptsprache in der Sandbox wie bei [[add-ons|Add-ons]]), und wie Trefferzonen mit einem Shape-Cast pro Zauber zusammenpassen.

Nächster Schritt: nach M6 einen ersten Boss entwerfen und daran das Datenformat festlegen.
