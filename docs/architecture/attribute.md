---
title: "Attribute — wie stark und fähig ein Magier gerade ist"
---
# Attribute — wie stark und fähig ein Magier gerade ist

Jeder Magier hat mehrere Eigenschaften, die zusammen sagen, wie stark und fähig er **in diesem Moment** ist. Effekte ändern sie zeitweise. Entschieden in [[adr-0016-attribute-mit-modifikatoren|ADR-0016]] (vorgeschlagen).

## Die Attribute (erster Satz)

| Attribut | Wirkung | Beispiel-Modifikator |
| --- | --- | --- |
| Lebenspunkte (max.) | Obergrenze der Gesundheit | Schild-Zauber +20 % |
| Mana (max.) und Mana-Regeneration | wie viel und wie schnell gezaubert werden kann | Erschöpfung −50 % Regeneration |
| Bewegungstempo | Geschwindigkeit von `MoveAndSlide` | Slow −40 % |
| Zauberkraft | Faktor auf ausgeteilten Schaden | Ermächtigung +25 % |
| Widerstand | Faktor auf erlittenen Schaden | Schwäche −20 % |
| Zaubertempo | Faktor auf Cooldowns und Wirkzeiten | Eile +30 % |
| Tarnung | wie schwer der Magier zu sehen ist | Unsichtbarkeit ([[sichtbarkeit\|Sichtbarkeit]]) |
| Wahrnehmung | wie gut der Magier Getarnte entdeckt | Hellsicht |

Der Satz ist erweiterbar: ein neues Attribut ist eine Enum-Variante, ein Basiswert in der Balance und eine Stelle, die den effektiven Wert liest.

## Wie gerechnet wird

1. **Basiswerte** kommen aus der Balance-Ressource (später je Klasse oder Ausrüstung).
2. **Modifikatoren** stehen in einer Liste am Magier: Attribut, additiv oder multiplikativ, Wert, Restdauer in Ticks, Quelle (`SimId`).
3. Das Set **Werte** (direkt nach Intent) zählt Restdauern herunter, entfernt abgelaufene Modifikatoren und rechnet: *effektiv = (Basis + Σ additiv) × Π (1 + multiplikativ)*, geklemmt auf Unter- und Obergrenzen.
4. Gerechnet wird **ganzzahlig in Tausendsteln**; die Reihenfolge der Modifikatoren ist nach `SimId` der Quelle festgelegt. So bleibt alles deterministisch.
5. Alle Systeme lesen nur `EffectiveAttributes`. Ein im Tick neu hinzugefügter Modifikator (etwa Slow aus dem Set Schaden) wirkt ab dem nächsten Tick.

## Stapelregeln

Ein Effekt legt fest, ob er sich mit sich selbst stapelt oder nur die Dauer erneuert (Standard: erneuern, nicht stapeln). Gleiche Effekte verschiedener Quellen stapeln sich, bis zu einer Obergrenze in der Balance.

## Wer die Werte sieht

HUD, KI, RL und Add-ons lesen die Attribute über die [[beobachtung|Beobachtung]]: die eigenen vollständig, die sichtbarer Gegner nur, soweit das Spiel sie zeigen will (Vorschlag: Leben als Anteil und sichtbare Effekte, keine genauen Werte).

## Was offen ist

- Endgültige Liste, Grenzen und Balance: wird mit KI-Selbstspiel ermittelt ([[ki-und-rl|KI & Reinforcement Learning]]).
- Gibt es dauerhafte Fortschritte (Stufen, Ausrüstung) oder nur Effekte innerhalb einer Runde? Bewusst nach dem Prototyp.
