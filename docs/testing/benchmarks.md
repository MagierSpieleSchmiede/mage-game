---
title: "Benchmarks — damit das Spiel auch auf alten Rechnern läuft"
---
# Benchmarks — damit das Spiel auch auf alten Rechnern läuft

Das Spiel soll auch auf alten Rechnern und Mobilgeräten flüssig laufen. Deshalb wird Leistung von Anfang an gemessen, mit festen Szenarien und Budgets. Gerüst in M1, vollständig in M4.

## Szenarien

Alle Szenarien sind deterministisch (fester Seed, aufgezeichnete Intents) und dieselben, die auch Tests und RL benutzen ([[teams|Teams]]).

| Szenario | Inhalt | misst vor allem |
| --- | --- | --- |
| `duel_idle` | 1-gegen-1, niemand handelt | Grundkosten eines Ticks |
| `duel_combat` | 1-gegen-1, alle Zauber im Dauerfeuer | Casting, Treffer, Schaden |
| `teams_5v5` | 5-gegen-5, Kampf | Sichtbarkeit, Zielwahl, Trefferprüfung mit vielen Spielern |
| `projectile_storm` | 4-gegen-4, 300 gleichzeitige Projektile und Fragmente | Shape-Casts, Spawn und Despawn |
| `stealth_5v5` | 5-gegen-5, die Hälfte unsichtbar | Sichtlinien-Raycasts |

## Was gemessen wird

| Messung | Werkzeug | Budget (Platzhalter, wird mit [[p-008-mindest-hardware-und-leistungsbudget\|P-008]] festgelegt) |
| --- | --- | --- |
| Simulations-Tick | Benchmark-Bibliothek in `mage_sim/benches/` | `teams_5v5` ≤ 2 ms pro Tick auf dem Referenzrechner (bei 60 Ticks/s also ≤ 12 % eines Kerns) |
| Schritte pro Sekunde headless | dieselben Benchmarks | `duel_combat` ≥ 50-fache Echtzeit auf einem Kern (für RL) |
| Snapshot und Beobachtung | Benchmarks | Snapshot von `teams_5v5` ≤ 0,2 ms, Größe ≤ 16 KB |
| Bildzeit Client | Benchmark-Modus des Clients: fährt eine feste Kamerafahrt über ein Szenario, schreibt Bildzeiten | 60 Bilder/s (16,7 ms) im 95. Perzentil auf dem Referenzrechner bei 1280 × 720 |
| Speicher | Benchmark-Modus | ≤ 1 GB Arbeitsspeicher im Client |
| Bandbreite | Netzwerk-Tests ([[synchronisation-latenz\|Synchronisation & Latenz]]) | ≤ 64 kbit/s pro Client in `teams_5v5` |

## Regeln

- **Referenzrechner:** ein konkretes, altes Gerät ([[p-008-mindest-hardware-und-leistungsbudget|P-008]]). Budgets gelten dort; auf schnellen Rechnern zählen nur Veränderungen.
- **Vor und nach:** Wer Simulation oder Rendering anfasst, vergleicht die Benchmarks mit dem Stand vor der Änderung und schreibt Verschlechterungen über 10 % in den Commit oder ins Implementation-Log.
- **CI** führt die Simulations-Benchmarks in kurzer Form aus, damit sie nicht verrotten; echte Messungen nur auf festen Geräten, weil CI-Rechner schwanken.
- **Stellschrauben für alte Geräte** gehören in die Einstellungen, nicht in die Simulation: Render-Auflösung des Low-Res-Targets, Schatten, Effekte, Bildrate. Die Simulation läuft überall gleich.
