---
title: "ADR-0020 — Cargo-Workspace mit der Simulation als eigener Crate ohne Render-Abhängigkeiten"
---
# ADR-0020 — Cargo-Workspace mit der Simulation als eigener Crate ohne Render-Abhängigkeiten

**Status:** angenommen (2026-11-14, mit Daniels Start der Implementierung)

**Kontext:** Dieselbe Simulation soll im Spiel-Client, auf dem Server, im RL-Training (viele Instanzen, schneller als Echtzeit), in Benchmarks und auf allen Plattformen inklusive Web laufen. Die Trennung Simulation/Präsentation war bisher nur eine Konvention zwischen Plugins in einer Crate.

**Entscheidung:** Das Repo wird ein Cargo-Workspace. `mage_sim` (Bibliothek) enthält alle Simulations-Plugins und hängt nur an Bevy-Teilen ohne Fenster, Renderer und Audio (`bevy_app`, `bevy_ecs`, `bevy_math`, `bevy_time`, `bevy_transform` …) sowie an Avian ohne Debug-Darstellung. `mage_client` (Spiel) bringt Präsentation, Eingabe, HUD und Sprachen. Später kommen `mage_server`, eine Crate für die RL-Brücke und eine für Add-ons dazu. Der Compiler erzwingt damit die Invariante „die Simulation liest niemals Präsentationszustand“.

**Verworfene Alternative:** Eine Crate mit Cargo-Features für headless. Verworfen, weil Features additiv sind und eine versehentlich eingeschleppte Render-Abhängigkeit erst spät auffällt.

**Konsequenzen:** Etwas mehr Verwaltungsaufwand (mehrere `Cargo.toml`). Die Bevy-Features werden in jeder Crate einzeln gewählt. CI prüft `mage_sim` auch für `wasm32-unknown-unknown`. Details: [[projektstruktur|Projektstruktur]].
