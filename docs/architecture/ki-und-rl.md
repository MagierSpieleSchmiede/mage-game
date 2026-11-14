---
title: "KI & Reinforcement Learning — Computergegner und Balancing durch Selbstspiel"
---
# KI & Reinforcement Learning — Computergegner und Balancing durch Selbstspiel

Später sollen KI-Spieler mitspielen, und mit Reinforcement Learning (RL) oder ähnlichen Verfahren wollen wir Computergegner trainieren und die Balance prüfen. Die Architektur ist dafür ausgelegt; gebaut wird es in Milestone 6.

## Was es dafür braucht, und wo es herkommt

| Bedarf | Lösung |
| --- | --- |
| KI steuert einen Magier wie ein Mensch | KI ist ein Intent-Produzent: sie liefert `MageIntent` pro Tick, wie Tastatur oder Netzwerk |
| KI sieht nur, was ein Spieler sieht | sie bekommt die [[beobachtung\|Beobachtung]] ihres Spielers, mit Sichtbarkeitsfilter ([[adr-0019-eine-beobachtungs-schnittstelle\|ADR-0019]]) |
| Training schneller als Echtzeit, viele Runden parallel | `mage_sim` läuft headless ohne Fenster ([[adr-0020-simulation-als-eigene-crate\|ADR-0020]]); ein Tick ist ein Funktionsaufruf, keine Wartezeit; viele Instanzen in mehreren Prozessen |
| Reproduzierbare Episoden | Determinismus: Szenario + Seed + Intents ergeben immer denselben Verlauf |
| Belohnungen | aus den Ereignissen (`DamageApplied`, `MageDied`, Rundenende); die Belohnungsformel lebt in der RL-Umgebung, nicht in der Simulation |
| Balance-Werte ändern ohne neu zu kompilieren | Balance und Zauber-Blueprints sind Daten (serde), die pro Runde geladen werden |

## Die Umgebung

Eine RL-Umgebung umhüllt `mage_sim`:

- `reset(szenario, seed)` → Beobachtungen aller KI-Spieler
- `step(intents)` → nächste Beobachtungen, Belohnungen, Ende ja/nein
- **Entscheidungsintervall:** eine KI entscheidet alle k Ticks (Platzhalter 4), dazwischen gilt ihr letzter Intent. Das spart Rechenzeit und entspricht menschlicher Reaktionszeit.
- **Mehrere Agenten:** Teams heißt Multi-Agent; jede `PlayerId` ist ein Agent.
- **Aktionsraum:** der `MageIntent` (Bewegung, Zielrichtung, Knöpfe); eine vereinfachte, diskretisierte Variante kann die Umgebung anbieten.

Wie die Umgebung nach Python kommt: [[p-013-rl-bruecke|P-013]].

## Computergegner im Spiel

- Trainierte Modelle werden in ein Austauschformat exportiert und im Spiel mit einer reinen Rust-Inferenz ausgeführt, damit sie auf allen [[plattformen|Plattformen]] laufen.
- Daneben einfache regelbasierte Gegner (Verhaltensbäume) als Grundlage, Rückfall und Trainingspartner.
- **Schwierigkeit** über Reaktionszeit, Zielgenauigkeit und Entscheidungsintervall des Agenten, nie über Vorteile in der Simulation. KI schummelt nicht.

## Balancing

- Selbstspiel-Turniere vieler Agenten mit verschiedenen Zauber-Sets; ausgewertet werden Siegquoten je Zauber, Attribut und Team-Zusammensetzung.
- Parameter-Suche: Balance-Werte automatisch variieren und Spiele simulieren, um Ausreißer zu finden (zu starke Zauber, nutzlose Attribute).
- Ergebnisse werden als Vorschlag in den Vault geschrieben; die Balance-Werte ändert ein Mensch.

## Was offen ist

- [[p-013-rl-bruecke|P-013]] Brücke zu Python, Multi-Agent-Schnittstelle, Export der Modelle
- Schritte pro Sekunde als Ziel: [[benchmarks|Benchmarks]]
