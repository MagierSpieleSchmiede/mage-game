---
title: "ADR-0019 — Eine Beobachtungs-Schnittstelle für HUD, KI, Reinforcement Learning und Add-ons"
---
# ADR-0019 — Eine Beobachtungs-Schnittstelle für HUD, KI, Reinforcement Learning und Add-ons

**Status:** vorgeschlagen

**Kontext:** Mehrere Verbraucher wollen wissen, was ein Spieler gerade weiß: das HUD, KI-Gegner, RL-Training, Add-ons, die den Charakter auslesen oder automatisieren, und später der Netzwerk-Server, der pro Client filtert. Bekäme jeder direkten Zugriff auf die ECS-Welt, sähe die KI durch Wände und Add-ons wären Cheats.

**Entscheidung:** Es gibt genau eine Funktion, die aus dem Simulationszustand eine **Beobachtung** (`Observation`) für einen `PlayerId` erzeugt: eigener Zustand (effektive Attribute, Leben, Mana, Cooldowns), alle Entities mit gesetztem Bit des eigenen Teams in `VisibleTo` (relativ zum Spieler, mit `SimId`), Ereignisse des letzten Ticks, die der Spieler wahrnehmen konnte. Die Beobachtung ist serde-serialisierbar und versioniert. Mensch, KI, RL und Add-ons bekommen dieselbe Sicht; schreiben dürfen alle nur `MageIntent`.

**Verworfene Alternative:** Eigene Sonderzugänge pro Verbraucher (KI liest Components, Add-ons bekommen eine eigene API). Verworfen, weil jede Sonderschnittstelle ein eigenes Leck für versteckte Information wäre und KI-Gegner sonst unfair stark würden.

**Konsequenzen:** KI-Gegner schummeln nicht; ihre Stärke wird über Reaktionszeit und Zielgenauigkeit des Agenten eingestellt. Die Beobachtung ist auch das Eingabeformat fürs RL-Training. Details: [[beobachtung|Beobachtung]].
