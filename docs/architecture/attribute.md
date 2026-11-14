---
title: "Attribute — wie stark und fähig ein Magier gerade ist"
---
# Attribute — wie stark und fähig ein Magier gerade ist

Jeder Magier hat mehrere Eigenschaften, die zusammen sagen, wie stark und fähig er **in diesem Moment** ist. Effekte ändern sie zeitweise. Entschieden in [[adr-0016-attribute-mit-modifikatoren|ADR-0016]].

## Die Attribute (erster Satz)

| Attribut | Wirkung | Beispiel-Modifikator |
| --- | --- | --- |
| Lebenspunkte (max.) | Obergrenze der Gesundheit | Schild-Zauber +20 % |
| Mana (max.) und Mana-Regeneration | wie viel und wie schnell gezaubert werden kann | Erschöpfung −50 % Regeneration |
| Bewegungstempo | Geschwindigkeit von `MoveAndSlide` | Slow −50 % |
| Zauberkraft | Faktor auf ausgeteilten Schaden | Ermächtigung +25 % |
| Widerstand | Faktor auf erlittenen Schaden | Schwäche −20 % |
| Zaubertempo | Faktor auf Cooldowns und Wirkzeiten | Eile +30 % |
| Tarnung | wie schwer der Magier zu sehen ist | Unsichtbarkeit ([[sichtbarkeit\|Sichtbarkeit]]) |
| Wahrnehmung | wie gut der Magier Getarnte entdeckt | Hellsicht |

Der Satz ist erweiterbar: ein neues Attribut ist eine Enum-Variante, ein Basiswert in der Balance und eine Stelle, die den effektiven Wert liest. Im Code: `crates/mage_sim/src/attributes.rs`, Basiswerte und Grenzen in `balance.rs`; jede Einheit bringt über die Marker-Component `Unit` ihre Modifikatoren und `EffectiveAttributes` mit ([[einheiten|Einheiten]]).

## Wie gerechnet wird

1. **Basiswerte** kommen aus der Balance-Ressource bzw. dem Blueprint der Kreatur; Ausrüstung wirkt später als Modifikator ohne Ablauf und darf nur außer Kampf gewechselt werden ([[adr-0032-kampfzustand|ADR-0032]]).
2. **Modifikatoren** stehen in einer Liste am Magier: Attribut, additiv oder multiplikativ, Wert, Restdauer in Ticks, Quelle (`SimId`).
3. Das Set **Werte** (direkt nach Intent) entfernt abgelaufene Modifikatoren, rechnet *effektiv = (Basis + Σ additiv) × Π (1 + multiplikativ)*, geklemmt auf Unter- und Obergrenzen, und zählt danach die Restdauern herunter. Ein Modifikator mit Dauer N wirkt so in genau den N folgenden Ticks; ohne Dauer wirkt er, bis er entfernt wird (später Ausrüstung). Ein Faktor fällt nie unter null.
4. Gerechnet wird **ganzzahlig in Tausendsteln**, jeder Faktor-Schritt wird auf ganze Tausendstel abgeschnitten; die Faktoren werden in der Reihenfolge der `SimId` ihrer Quelle angewendet. Die Reihenfolge zählt wirklich: 1,001 × 1,333 × 0,75 ergibt 1,000, umgekehrt 0,999. So bleibt alles deterministisch.
5. Alle Systeme lesen nur `EffectiveAttributes`. Ein im Tick neu hinzugefügter Modifikator (etwa Slow aus dem Set Schaden) wirkt ab dem nächsten Tick.

## Stapelregeln

Ein Effekt (`EffectId`) legt fest, ob er sich mit sich selbst stapelt oder nur die Dauer erneuert (Standard: erneuern, nicht stapeln). Gleiche Effekte verschiedener Quellen stapeln sich, bis zu einer Obergrenze in der Balance (`max_stacks`, Platzhalter 3); ein weiterer wird dann ignoriert.

## Wer die Werte sieht

HUD, KI, RL und Add-ons lesen die Attribute über die [[beobachtung|Beobachtung]]: die eigenen vollständig, die sichtbarer Gegner nur, soweit das Spiel sie zeigen will (Vorschlag: Leben als Anteil und sichtbare Effekte, keine genauen Werte).

## Was offen ist

- Endgültige Liste, Grenzen und Balance: wird mit KI-Selbstspiel ermittelt ([[ki-und-rl|KI & Reinforcement Learning]]).
- Gibt es dauerhafte Fortschritte (Stufen, Ausrüstung) oder nur Effekte innerhalb einer Runde? Bewusst nach dem Prototyp.
