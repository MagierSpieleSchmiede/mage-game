---
title: "Spec 001 — Mage Game: fundamentaler Prototyp (revidiert)"
---
# Spec 001 — Mage Game: fundamentaler Prototyp (revidiert)

**Toolchain und Versionen**

- Rust Stable; Bevy 0.20; Avian3D 0.8.0 (bestätigt: unterstützt Bevy 0.20 explizit; auf crates.io geprüft).
- Bevy-Wissen aus der Zeit vor 0.20 ist nicht maßgeblich: Vor dem Schreiben von Code die Migrationsleitfäden 0.18→0.19 und 0.19→0.20 lesen und für jede unsichere API die echte Crate-Dokumentation oder den Quellcode prüfen. Keine Bevy-API aus dem Gedächtnis.
- Nach jedem Modul: `cargo check` und `cargo clippy -- -D warnings`; beides muss sauber sein, bevor weitergearbeitet wird.
- `Cargo.lock` wird eingecheckt; `rust-version` steht in `Cargo.toml`.
- Required Components und einfache `commands.spawn((...))`-Tupel. Keine Legacy-Bundles, kein `bsn!`.

**Umfang**

Entwirf für (implementiere nicht): KI-Gegner und Reinforcement Learning ([[ki-und-rl|KI & RL]]), Rollback-Netcode mit Server als Schiedsrichter ([[netzwerk|Netzwerk]], [[sicherheit|Sicherheit]]), Snapshot/Restore, Unsichtbarkeit mit Tarnung und Wahrnehmung ([[sichtbarkeit|Sichtbarkeit]]), Add-ons ([[add-ons|Add-ons]]), Touch-Steuerung, Mobile- und Web-Builds ([[plattformen|Plattformen]]), eine große Welt mit Interessenbereich ([[welt|Welt]]), Reittiere ([[reittiere|Reittiere]]), vom Spieler zusammengestellte Fähigkeitenleisten ([[faehigkeiten|Fähigkeiten & Belegung]]), weitere Deliveries, Modifikatoren, Attribute und Effekte. Nicht im Umfang: Netzwerk, KI, Add-ons, Menüs, Audio, Asset-Pipelines, Spielstände, Zauber-Skriptsprachen, Animationen aus Asset-Dateien. Keine Abstraktion, die nicht der „Entwirf für"-Liste dient oder mindestens zweimal verwendet wird. Keine TODO-Stubs für Kernfunktionalität.

**Simulations-Invarianten**

Die Invarianten auf der Seite [[simulationsmodell|Simulationsmodell]] (1 bis 9, dazu die vorgeschlagenen 10 bis 12) sind Teil dieser Spezifikation.

**Anforderungen**

1. Intent und Steuerung: `MageIntent` wie auf der Seite [[intent-steuerung|Intent & Steuerung]], inklusive des Flanken-Protokolls (Latch-Zähler im `InputBuffer`, Ableitung von `pressed` und Reset im Set Intent).
2. Zauber: Blueprint-Registry nach `SpellId`; Deliveries `Projectile` und `GroundArea` (Zielpunkt: Strahl-Boden-Schnitt, geklemmt auf Maximalreichweite; Fuse in Ticks); Modifikatoren `Spread` (on cast), `Homing` (per tick, Zielverlust ohne LOS), `Split` (on impact); Effekte `Damage`, `AreaDamage`, `Slow`; drei Zauber: Bolt, Homing-Bolt, Feuerball, dazu der Fragment-Blueprint; geteilt werden der Modifikator `Spread` und der Effekt `Damage`.
3. Kampf und Physik: Health, Mana mit Regeneration, Cooldowns pro Slot; kinematischer Magier über Avians `MoveAndSlide`; Projektile simulationsbewegt mit Shape-Cast pro Tick; Hindernisse blockieren Projektile, Homing-Sichtlinien und Flächenschaden; Observer `SpellImpact`, `DamageApplied`, `MageDied` als die einen Codepfade; toter Magier behält seine Entity (`Dead`-Marker), Respawn nach 180 Ticks; jede ergebnisrelevante Reihenfolge nach `SimId`.
4. Arena und Präsentation: Greybox-Arena (Boden, Wände, Hindernisse unterschiedlicher Höhe, Spawnpunkte je Team); Third-Person-Orbitkamera, `aim_dir` aus der Kamera; Low-Res-Render-Target mit Nearest-Upscale, Auflösung als Einstellung; Transform-Interpolation zwischen fixen Ticks; HUD aus der Beobachtung mit Leben, Mana, Effekten und Slot-Cooldowns; Animation nur als Präsentation, die nie einen Cast verzögert.
5. Teams und Attribute: beliebig viele Magier in beliebig vielen Teams ([[teams|Teams]]); getestet werden 1-gegen-1 und 2-gegen-2. Attribute mit Modifikatoren und Set Werte ([[attribute|Attribute]]); Slow ist ein Modifikator.
6. Sichtbarkeit und Beobachtung in Grundform: `VisibleTo` pro Team aus Sichtweite und Sichtlinie, Beobachtung pro Spieler; Homing und HUD lesen nur diese ([[sichtbarkeit|Sichtbarkeit]], [[beobachtung|Beobachtung]]).
7. Eingabe und Sprachen: Tastatur und Maus sowie Controller über Aktionen ([[eingabegeraete|Eingabegeräte]]); alle Texte in Englisch und Deutsch ([[lokalisierung|Lokalisierung]]).
8. Plattformen und Leistung: Workspace mit `mage_sim` ohne Render-Abhängigkeiten; `mage_sim` baut für alle Ziele aus [[plattformen|Plattformen]]; Benchmarks nach [[benchmarks|Benchmarks]].

**Vorgehen**

1. Erst die Dokumentation: Dieser Vault ersetzt das ursprünglich geforderte `DESIGN.md`. Module und Plugins, Set-Kette, Zauber-Datenmodell und alle Entscheidungen mit verworfener Alternative stehen hier.
2. Dann Item für Item aus dem [[backlog|Backlog]], **Tests zuerst** ([[adr-0026-tests-zuerst|ADR-0026]]), kompilierend und mit sauberem Clippy nach jedem Item.
3. Abschluss: [[erweiterungspunkte|Erweiterungspunkte]] pflegen, Implementation-Logs je Milestone ergänzen.

**Abnahmekriterien**

- `cargo check`, `cargo clippy -- -D warnings` und `cargo test` laufen sauber.
- `cargo run` öffnet eine spielbare Arena: Tastatur und Maus oder Controller, Slots wie auf [[eingabegeraete|Eingabegeräte]]; Dummies nehmen Schaden, sterben, respawnen; Texte auf Englisch und Deutsch umschaltbar.
- 2-gegen-2-Test grün; Sichtbarkeitstest „kein Gegner hinter der Wand in der Beobachtung“ grün; Test „Animation blockiert keinen Cast“ grün.
- `mage_sim` baut für alle Ziele aus [[plattformen|Plattformen]]; Benchmarks laufen und ihre Ergebnisse stehen im Implementation-Log von M4.
- Headless-Integrationstest: App nur aus Simulations-Plugins; `MageIntent`, der den Bolt auf den Dummy wirkt; eine feste Anzahl Ticks, ohne Wallclock getrieben (Hilfsfunktion, siehe [[headless-integrationstest|Headless-Integrationstest]]); fester Seed und kurzer Abstand, damit die Streuung den Treffer nicht verhindert; Assertion: Dummy hat HP verloren, Zaubernder hat Mana ausgegeben.
- Determinismustest: dieselbe Intent-Sequenz zweimal vom selben Seed; beide Läufe werden serialisiert und Byte-für-Byte verglichen; gültig, weil `Spread` den geseedeten RNG verbraucht.
- README: Steuerung und Test-Befehle.
