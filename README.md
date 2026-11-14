![Stil-Vorbild: ein Magier mit leuchtendem Stab bei Nacht, Vollmond über einem See](docs/gestaltung/bilder/vorbild.jpg)

# Mage Game

**Ein stilisiertes 3D-Arena-Spiel in Rust: Teams von Magiern, freie Bewegung, modulare Zauber.**

[![Docs](https://img.shields.io/badge/docs-vault-7bd88f)](https://magierspieleschmiede.github.io/mage-game/)
[![License: MIT](https://img.shields.io/badge/license-MIT-lightgrey)](LICENSE)

> [!WARNING]
> **Mage Game ist ein Prototyp in der Planungsphase.** Die Spezifikation steht,
> die Implementierung hat noch nicht begonnen: `cargo run` gibt heute nur
> „Hello, world!“ aus. Alles hier kann sich ohne Vorwarnung ändern.

## Die Idee

Zwei oder mehr Teams von Magiern treten in einer Arena gegeneinander an,
vom 1-gegen-1 bis zu mehreren Teams mit mehreren Spielern. Die Magier bewegen
sich frei in 3D, zielen aus der Third-Person-Kamera und wirken Zauber, die aus
Bausteinen zusammengesetzt sind: ein **Delivery** (wie der Zauber ans Ziel
kommt), **Modifikatoren** (Streuung, Zielsuche, Aufsplitten) und **Effekte**
(Schaden, Flächenschaden, Verlangsamung). Jeder Magier hat **Attribute** wie
Tempo, Zauberkraft, Widerstand oder Tarnung, die zeigen, wie stark er gerade
ist, und die Effekte im Kampf verändern. Magier können sich **unsichtbar**
machen; was ein Team nicht sieht, erfährt es auch nicht.

Darunter liegt eine Wette: Das gesamte Gameplay läuft als **deterministische
Tick-Simulation**, headless-fähig und ohne Abhängigkeit von der Frame-Zeit.
Gleicher Startzustand, gleiche Eingaben und gleicher Seed ergeben denselben
Endzustand, Byte für Byte. Das macht das Spiel testbar ohne Fenster und ist die
Grundlage für alles, was danach kommt:

- **KI-Gegner**, trainiert mit **Reinforcement Learning**, die auch beim
  Balancing helfen.
- **PvP über einen Server**, der allein entscheidet, mit Vorhersage auf den
  Clients ([Sicherheit](docs/architecture/sicherheit.md)).
- **Add-ons**, die den eigenen Charakter auslesen oder automatisieren, in
  einer Sandbox.

Geschrieben in **Rust** mit [Bevy](https://bevy.org) 0.20 und
[Avian](https://github.com/avianphysics/avian) 0.8 (Physik-Queries).

## Plattformen und Sprachen

- **Windows, Linux, macOS**, außerdem **Android und iOS**; im Browser, wenn
  das ohne größere Probleme geht.
- **Englisch und Deutsch** von Anfang an.
- **Tastatur und Maus** oder **Controller**, auf Mobilgeräten Touch.
- Läuft auch auf **alten Rechnern**: Leistung wird mit einer Benchmark-Suite
  gegen ein festes Budget gemessen.

## Was heute funktioniert

Noch nichts Spielbares. Fertig ist die Planung: Spezifikation, Architektur,
Entscheidungen (ADRs), Teststrategie, Problem-Log und ein Backlog mit den
nächsten Aufgaben, siehe [Dokumentation](#dokumentation).

## Geplant

| Milestone | Inhalt |
| --- | --- |
| M1 — Walking Skeleton | Simulation headless: Teams, Intent, Attribute, Bolt, Mana, Cooldowns; Tests zuerst; CI für alle Plattformen |
| M2 — Zauber-Matrix | Homing-Bolt, Feuerball, Slow, Tod und Respawn, Sichtbarkeit in Grundform |
| M3 — Arena & Präsentation | Greybox-Arena, Kamera, Low-Res-Look, HUD, Maus und Controller, Englisch und Deutsch |
| M4 — Härtung & Messung | Benchmarks, Desktop-Builds, Determinismus über alle Zauber |
| M5 — Unsichtbarkeit | Tarnung, Wahrnehmung, Enthüllung |
| M6 — KI & RL | Computergegner, Trainingsumgebung, Balancing durch Selbstspiel |
| M7 — Netzwerk & Sicherheit | Server, Client-Vorhersage, Sync- und Latenztests |
| M8 — Add-ons | Sandbox, Auslesen und Automatisieren |
| M9 — Mobile & Web | Android, iOS, Touch, Web-Client wenn machbar |

Die konkreten nächsten Aufgaben: [Backlog](docs/backlog.md).

## Steuerung (geplant, ab M3)

| Aktion | Tastatur und Maus | Controller |
| --- | --- | --- |
| Bewegen | `W` `A` `S` `D` | linker Stick |
| Kamera und Zielen | Maus | rechter Stick |
| Bolt | `1` oder linke Maustaste | rechter Trigger |
| Homing-Bolt | `2` | rechte Schultertaste |
| Feuerball | `3` | linker Trigger |

Alle Belegungen sind änderbar.

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
im Problem-Log, Pläne als Milestones, die nächsten Aufgaben im
[Backlog](docs/backlog.md).

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

## Sicherheit

Wie Server und PvP abgesichert werden: [Sicherheit](docs/architecture/sicherheit.md).
Sicherheitslücken bitte nicht öffentlich melden, siehe [SECURITY.md](SECURITY.md).

## Lizenz

**MIT**, siehe [LICENSE](LICENSE).
