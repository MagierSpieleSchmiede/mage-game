---
title: "Sichtbarkeit — Unsichtbarkeit als Mechanik, pro Team berechnet"
---
# Sichtbarkeit — Unsichtbarkeit als Mechanik, pro Team berechnet

Magier können sich unsichtbar machen. Damit das fair bleibt, ist Sichtbarkeit kein Grafikeffekt, sondern Simulationszustand: Was ein Team nicht sieht, erfährt es auch nicht, weder über die Grafik noch über KI, Add-ons oder das Netzwerk. Entschieden in [[adr-0018-sichtbarkeit-als-simulationszustand|ADR-0018]] (vorgeschlagen).

## Regeln

Jede Simulations-Entity trägt eine Bitmaske `VisibleTo` (ein Bit pro Team). Das Set **Sichtbarkeit** am Ende jedes Ticks setzt sie neu:

1. Eigene Teammitglieder und deren Zauber sind immer sichtbar.
2. Ein gegnerischer Magier ist für Team T sichtbar, wenn mindestens ein Magier aus T ihn **in Sichtweite** hat, eine **Sichtlinie** besteht (Raycast gegen die statische Arena) und seine **Tarnung** die **Wahrnehmung** des Beobachters auf diese Entfernung nicht übersteigt.
3. **Enthüllung:** Wer zaubert, Schaden nimmt oder Schaden verursacht, ist für eine kurze Zeit (Platzhalter 30 Ticks) für alle sichtbar, unabhängig von Tarnung.
4. **Zauber** sind sichtbar, wenn sie in Sichtweite und Sichtlinie eines Teammitglieds sind (Vorschlag: auch Zauber unsichtbarer Magier, damit man reagieren kann).
5. Tote Magier sind für alle sichtbar.

Tarnung und Wahrnehmung sind [[attribute|Attribute]]; Unsichtbarkeit ist ein Effekt, der die Tarnung für eine Dauer stark erhöht.

## Wer die Maske benutzt

| Verbraucher | Verhalten |
| --- | --- |
| Homing ([[zauber-datenmodell\|Zauber-Datenmodell]]) | wählt nur Ziele, die für das Team des Zaubernden sichtbar sind; verliert das Ziel, wenn es unsichtbar wird |
| [[beobachtung\|Beobachtung]] (HUD, KI, Add-ons) | enthält nur Entities mit gesetztem Bit des eigenen Teams |
| Netzwerk ([[netzwerk\|Netzwerk]]) | der Server schickt einem Client nur, was dessen Team sieht (Interest Management) |
| Darstellung | eigene unsichtbare Teammitglieder halbtransparent; Gegner, die nicht sichtbar sind, existieren im Client gar nicht |
| Replays und Zuschauer | sehen alles |

## Kosten

Pro Tick höchstens (Teams × Magier × gegnerische Magier) Sichtlinien-Raycasts; vorher werden Paare außerhalb der Sichtweite über eine räumliche Abfrage aussortiert. Gemessen im Benchmark-Szenario „5-gegen-5“ ([[benchmarks|Benchmarks]]).

## Was offen ist

- Feinheiten der Balance (Sichtweite, Enthüllungsdauer, Kosten der Unsichtbarkeit): Platzhalter, später durch KI-Selbstspiel abgestimmt.
- Geräusche und Spuren (etwa Schritte, die Unsichtbare verraten) wären ebenfalls Information und müssten durch denselben Filter.
- Darstellung des „Verschwindens“ ohne Information zu verraten (die letzte bekannte Position bleibt kurz stehen und verblasst).
