![Stil-Vorbild: ein Magier mit leuchtendem Stab bei Nacht, Vollmond über einem See](docs/gestaltung/bilder/vorbild.jpg)

# Mage Game

**Ein stilisiertes 3D-Arena-Duellspiel in Rust: zwei Magier, freie Bewegung, modulare Zauber.**

[![Docs](https://img.shields.io/badge/docs-vault-7bd88f)](https://magierspieleschmiede.github.io/mage-game/)
[![License: MIT](https://img.shields.io/badge/license-MIT-lightgrey)](LICENSE)

> [!WARNING]
> **Mage Game ist ein Prototyp in der Planungsphase.** Die Spezifikation steht,
> die Implementierung hat noch nicht begonnen: `cargo run` gibt heute nur
> „Hello, world!“ aus. Alles hier kann sich ohne Vorwarnung ändern.

## Die Idee

Zwei Magier stehen sich in einer Arena gegenüber. Sie bewegen sich frei in 3D,
zielen aus der Third-Person-Kamera und wirken Zauber, die aus Bausteinen
zusammengesetzt sind: ein **Delivery** (wie der Zauber ans Ziel kommt),
**Modifikatoren** (Streuung, Zielsuche, Aufsplitten) und **Effekte** (Schaden,
Flächenschaden, Verlangsamung). Ein neuer Zauber ist eine neue Kombination,
kein neuer Code.

Darunter liegt eine Wette: Das gesamte Gameplay läuft als **deterministische
Tick-Simulation** im `FixedUpdate`, headless-fähig und ohne Abhängigkeit von
der Frame-Zeit. Gleicher Startzustand, gleiche Eingaben und gleicher Seed
ergeben denselben Endzustand, Byte für Byte. Das macht das Spiel testbar ohne
Fenster und legt die Grundlage für spätere KI-Gegner und PvP mit
Rollback-Netcode.

Geschrieben in **Rust** mit [Bevy](https://bevy.org) 0.20 und
[Avian](https://github.com/avianphysics/avian) 0.8 (Physik-Queries).

## Was heute funktioniert

Noch nichts Spielbares. Fertig ist die Dokumentation: Spezifikation,
Architektur, 14 Entscheidungen (ADRs), Teststrategie und ein Problem-Log,
siehe [Dokumentation](#dokumentation).

## Geplant

| Milestone | Inhalt |
| --- | --- |
| M1 — Walking Skeleton | Simulation headless: zwei Magier, Intent, Bolt, Mana, Cooldowns, Headless- und Determinismustest |
| M2 — Zauber-Matrix | Homing-Bolt, Feuerball (Fläche, Split, Fragmente), Slow, Tod und Respawn |
| M3 — Arena & Präsentation | Greybox-Arena, Orbitkamera, Low-Res-Look, HUD: `cargo run` spielbar gegen einen Dummy |
| M4 — Härtung | Determinismustest über alle Zauber, Clippy sauber, Doku nachgezogen |

Bewusst **nach** dem Prototyp: KI-Gegner, Netzwerk-PvP mit Rollback, weitere
Zauber-Bausteine. Die Architektur ist dafür entworfen, implementiert wird es
noch nicht.

## Steuerung (geplant, ab M3)

| Taste | Aktion |
| --- | --- |
| `W` `A` `S` `D` | Bewegen |
| Maus | Kamera drehen und zielen |
| `1` | Bolt |
| `2` | Homing-Bolt |
| `3` | Feuerball |

## Bauen und testen

Voraussetzung: Rust stable ≥ 1.97.1 über [rustup](https://rustup.rs). Unter
Windows zusätzlich die Visual Studio Build Tools (C++-Workload).

```sh
cargo run       # das Spiel starten (der erste Build dauert 5–20 Minuten)
cargo test      # alle Tests
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Linux braucht für Bevy die üblichen System-Pakete (ALSA, udev, X11/Wayland),
siehe [Bevys Linux-Abhängigkeiten](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md).

## Dokumentation

Planung und Entscheidungen leben im **Vault** unter [`docs/`](docs/index.md):
Markdown-Seiten, die man mit [Obsidian](https://obsidian.md) öffnet
(*Open folder as vault* → `docs`) und die genauso über Git laufen wie der
Code. Eine Seite pro Thema, Entscheidungen als nummerierte ADRs, offene Fragen
im Problem-Log, Pläne als Milestones.

Zum Lesen gibt es den Vault als Webseite:
**<https://magierspieleschmiede.github.io/mage-game/>**. Sie wird bei jedem
Push auf `master`, der `docs/` ändert, neu gebaut. Lokal ansehen (braucht git
und Node ≥ 22):

```sh
site/build.sh --serve    # http://localhost:8080, baut bei jeder Änderung neu
```

## Mitmachen

Neu dabei? Die Anleitung
[Einstieg für Mitwirkende (Windows)](docs/einstieg-windows.md) führt von null
bis zum ersten Beitrag: Git, GitHub Desktop, Rust, Obsidian und der
Arbeitsablauf. Danach je nach Interesse:
[KI-Agenten](docs/einstieg-ki-agenten.md) (Claude Code, Codex) oder
[Gestalten](docs/einstieg-gestalten.md) (Blender, ArtCraft).

Für Code-Agenten (Claude Code, Codex & Co.) gilt [AGENTS.md](AGENTS.md).

## Lizenz

**MIT**, siehe [LICENSE](LICENSE).
