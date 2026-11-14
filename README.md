![Stil-Vorbild: ein Magier mit leuchtendem Stab bei Nacht, Vollmond über einem See](docs/gestaltung/bilder/vorbild.jpg)

# Mage Game

**Ein stilisiertes 3D-Arena-Spiel in Rust: Teams von Magiern, freie Bewegung, modulare Zauber.**

[![Docs](https://img.shields.io/badge/docs-vault-7bd88f)](https://magierspieleschmiede.github.io/mage-game/)
[![License: MIT](https://img.shields.io/badge/license-MIT-lightgrey)](LICENSE)

> [!WARNING]
> **Mage Game ist ein Prototyp in der Planungsphase.** Noch ist nichts
> spielbar. Was zuletzt erledigt wurde und was als Nächstes kommt, steht
> immer aktuell auf der Seite [Status](docs/status.md). Alles hier kann sich ohne Vorwarnung ändern.

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
- **PvP mit Rollback-Netcode** und einem Server als Schiedsrichter: Jeder
  Client rechnet selbst, was er sehen darf; im Zweifel gilt der Server
  ([Sicherheit](docs/architecture/sicherheit.md)).
- **Add-ons**, die den eigenen Charakter auslesen oder automatisieren, in
  einer Sandbox.
- **Kreaturen** neben den Spielern: wilde Tiere, Pets und Bosse.
- **Reittiere** am Boden und in der Luft, mit eigenem Leben, auf denen auch
  gekämpft wird.
- Eine **große Welt** für tausende Spieler, von der jeder Client nur die
  Umgebung seines Magiers lädt.

Geschrieben in **Rust** mit [Bevy](https://bevy.org) 0.20 und
[Avian](https://github.com/avianphysics/avian) 0.8 (Physik-Queries).

## Plattformen und Sprachen

- **Windows, Linux, macOS**, außerdem **Android und iOS**; im Browser, wenn
  das ohne größere Probleme geht.
- **Englisch und Deutsch** von Anfang an.
- **Tastatur und Maus** oder **Controller**, auf Mobilgeräten Touch.
- Läuft auch auf **alten Rechnern**: Leistung wird mit einer Benchmark-Suite
  gegen ein festes Budget gemessen.

## Stand und Planung

| Frage | Seite |
| --- | --- |
| Was wurde zuletzt erledigt, was kommt als Nächstes? | [Status](docs/status.md) |
| Welche Aufgaben sind dran? | [Backlog](docs/backlog.md) |
| Was kommt in welchem Milestone (M1 Walking Skeleton bis M9 Mobile & Web)? | [Roadmap](docs/milestones/roadmap.md) |
| Was ist entschieden, was nur vorgeschlagen? | [Entscheidungen](docs/decisions/entscheidungen.md) |
| Welche Fragen sind offen? | [Problem-Log](docs/problems/problem-log.md) |

Diese Seiten sind die einzige Quelle; das README wiederholt ihren Inhalt nicht.

## Steuerung (geplant, ab M3)

Standardbelegung, Kopie aus [Eingabegeräte](docs/architecture/eingabegeraete.md) (dort auch Touch):

| Aktion | Tastatur und Maus | Controller |
| --- | --- | --- |
| Bewegen | `W` `A` `S` `D` | linker Stick |
| Kamera und Zielen | Maus | rechter Stick |
| Slot 1 (Standard: Bolt) | `1` oder linke Maustaste | rechter Trigger |
| Slot 2 (Standard: Homing-Bolt) | `2` | rechte Schultertaste |
| Slot 3 (Standard: Feuerball) | `3` | linker Trigger |
| Slot 4 bis 12 | `4` … `0`, `Q`, `E` | Schultertaste + Knopf |

Alle Belegungen sind änderbar, und welche Fähigkeit in welchem Slot liegt, stellt jeder Spieler selbst zusammen.

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
Code. Eine Seite pro Thema, jede Tatsache an genau einer Stelle; Einstieg ist
die [Startseite](docs/index.md).

Zum Lesen gibt es den Vault als Webseite:
**<https://magierspieleschmiede.github.io/mage-game/>**. Sie wird bei jedem
Push auf `master`, der `docs/` ändert, neu gebaut. Zum Probelesen vor dem
Merge: auf den Branch `preview` pushen (jederzeit erlaubt, auch mit
`--force`); das Ergebnis erscheint unter
<https://magierspieleschmiede.github.io/mage-game/preview/>. Lokal ansehen (braucht git
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
