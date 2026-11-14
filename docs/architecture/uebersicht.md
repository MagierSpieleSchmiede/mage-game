---
title: "Übersicht — das System in einem Diagramm"
---
# Übersicht — das System in einem Diagramm

Alles Gameplay läuft in einer headless-fähigen Simulation im `FixedUpdate`. Die Präsentation ist ein reiner Beobachter: Sie erzeugt aus Eingabe und Kamera den `MageIntent` und hängt Visuals an Simulations-Entities, sobald deren Marker-Components auftauchen.

```mermaid
flowchart TD
  K["Tastatur und Maus"] --> HC["HumanController (Update)"]
  HC --> BUF["InputBuffer (Ressource, nach PlayerId)"]
  BUF --> S["Simulation (FixedUpdate, Tick-Kette)"]
  S --> Z["Simulationszustand (Components und Ressourcen)"]
  S --> EV["Events (SpellImpact, DamageApplied, MageDied)"]
  Z --> P["Präsentation (Kamera, Render, HUD)"]
  EV --> P
```

Zwei Regeln halten das System erweiterbar: Die Simulation kennt niemanden, der Intent erzeugt (Mensch, KI, Netzwerk sind austauschbar), und die Präsentation schreibt genau eine Sache zurück: den Intent.
