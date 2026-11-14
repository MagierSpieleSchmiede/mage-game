---
title: "Projektstruktur — Workspace, Crates und Plugins, und warum es sie gibt"
---
# Projektstruktur — Workspace, Crates und Plugins, und warum es sie gibt

Das Repo wird ein Cargo-Workspace ([[adr-0020-simulation-als-eigene-crate|ADR-0020]], vorgeschlagen). Die Grenze zwischen Simulation und Präsentation ist eine Crate-Grenze: Der Compiler verhindert, dass die Simulation Fenster, Renderer, Texte oder Eingabegeräte kennt.

## Crates

| Crate | Art | Inhalt | Ab |
| --- | --- | --- | --- |
| `mage_sim` | Bibliothek | alle Simulations-Plugins, Beobachtung, Szenarien, Snapshots; Bevy ohne Fenster/Render/Audio, Avian ohne Debug-Darstellung | M1 |
| `mage_client` | Programm | das Spiel: Präsentation, Eingabe, HUD, Lokalisierung, Animation | M1 (leer), M3 |
| `mage_server` | Programm | headless Server mit Netzwerk | M7 |
| RL-Brücke | Bibliothek | Umgebung für Reinforcement Learning ([[p-013-rl-bruecke\|P-013]]) | M6 |
| Add-on-Host | Bibliothek | Sandbox und Schnittstelle für Add-ons | M8 |

Benchmarks liegen in `mage_sim/benches/` (Simulation) und als Szenario-Modus im Client (Bildzeit), siehe [[benchmarks|Benchmarks]].

## Plugins in `mage_sim` (Simulation)

| Plugin | Verantwortung | Warum es existiert |
| --- | --- | --- |
| `controller` | `MageIntent` einfrieren, Flanken ableiten | Intent-Verarbeitung ist caster-unabhängig und testbar ohne Fenster |
| `attributes` | Basiswerte, Modifikatoren, effektive Werte (Set Werte) | ein Ort für „wie stark ist der Magier gerade“ ([[attribute\|Attribute]]) |
| `mage` | Gesundheit, Mana, Cooldowns, Bewegung, Team | ein Ort für alles, was einen Magier ausmacht |
| `spells` | Blueprint-Registry, Casting, Deliveries, Modifikatoren, Effekte | Konfiguration, Casting und Ausführung getrennt halten |
| `combat` | Treffer, Schaden, Tod, Respawn | Schaden, Mana und Tod laufen je durch genau einen Codepfad |
| `visibility` | `VisibleTo` pro Team (Set Sichtbarkeit) | Unsichtbarkeit fair für alle Verbraucher ([[sichtbarkeit\|Sichtbarkeit]]) |
| `observation` | Beobachtung pro Spieler | die eine Leseschnittstelle ([[beobachtung\|Beobachtung]]) |
| `arena` (Gameplay-Hälfte) | statische Kollider, Spawnpunkte je Team, LOS-Queries | die Welt als reine Abfragequelle, kein Zustandsbesitz |

## Plugins in `mage_client` (Präsentation)

| Plugin | Verantwortung | Warum es existiert |
| --- | --- | --- |
| `input` | Aktionen aus Tastatur/Maus, Controller, Touch; füllt `InputBuffer` | Eingabegeräte sind Präsentation ([[eingabegeraete\|Eingabegeräte]]) |
| `camera` | Third-Person-Orbit, berechnet `aim_dir` | Aim entsteht aus der Kamera, gehört aber in den Intent |
| `render` | Visuals an Marker-Entities, Low-Res-Target, Interpolation, Teamfarben | Simulations-Entities tragen keine Meshes, Materialien oder Lichter |
| `animation` | folgt dem Simulationszustand, nie umgekehrt | [[adr-0021-animation-ist-reine-praesentation\|ADR-0021]] |
| `hud` | Leben, Mana, Effekte, Cooldowns aus der Beobachtung | UI ist Reaktion, niemals Quelle |
| `i18n` | Fluent-Texte, Sprachwahl | [[lokalisierung\|Lokalisierung]] |
| `arena` (visuelle Hälfte) | Greybox-Meshes | dieselbe Arena einmal als Kollider, einmal als Bild |

Der Headless-Modus ist kein Nachgedanke: Die App aus `mage_sim` bauen und ticken ist der Normalfall von Tests, Benchmarks, Server und RL-Training.
