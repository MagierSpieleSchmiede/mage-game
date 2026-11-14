---
title: "Projektstruktur — das Plugin-Layout und warum jedes Plugin existiert"
---
# Projektstruktur — das Plugin-Layout und warum jedes Plugin existiert

Zwei Plugin-Gruppen, strikt getrennt: Die Präsentation darf Simulationszustand lesen, die Simulation liest niemals Präsentationszustand.

| Plugin | Gruppe | Verantwortung | Warum es existiert |
| --- | --- | --- | --- |
| `controller` | Simulation | `MageIntent` einfrieren, Flanken ableiten | Intent-Verarbeitung ist caster-unabhängig und testbar ohne Fenster |
| `mage` | Simulation | Gesundheit, Mana, Cooldowns, Bewegung des Magiers | ein Ort für alles, was einen Magier ausmacht |
| `spells` | Simulation | Blueprint-Registry, Casting, Deliveries, Modifikatoren, Effekte | Konfiguration, Casting und Ausführung getrennt halten |
| `combat` | Simulation | Treffer, Schaden, Status, Tod, Respawn | Schaden, Mana und Tod laufen je durch genau einen Codepfad |
| `arena` (Gameplay-Hälfte) | Simulation | statische Kollider der Arena, LOS-Queries | die Welt als reine Abfragequelle, kein Zustandsbesitz |
| `input` | Präsentation | tastet Tastatur und Maus ab, füllt `InputBuffer` | Intent-Erzeugung ist Präsentation, nicht Simulation |
| `camera` | Präsentation | Third-Person-Orbit, berechnet `aim_dir` | Aim entsteht aus der Kamera, gehört aber in den Intent, nicht in die Simulation |
| `render` | Präsentation | Visuals an Marker-Entities, Low-Res-Target, Transform-Interpolation | Simulations-Entities tragen keine Meshes, Materialien oder Lichter |
| `hud` | Präsentation | HP/Mana-Balken, Cooldown-Anzeige beider Magier | UI ist Reaktion, niemals Quelle |
| `arena` (visuelle Hälfte) | Präsentation | Greybox-Meshes für Boden, Wände, Hindernisse | dieselbe Arena einmal als Kollider, einmal als Bild |

Der Headless-Modus ist kein Nachgedanke: Die App aus reinen Simulations-Plugins bauen und ticken ist der Normalfall der Tests, nicht ein Spezialpfad.
