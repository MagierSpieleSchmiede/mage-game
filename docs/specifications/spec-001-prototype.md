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

Entwirf für (implementiere nicht): KI-Controller, vernetzte Spieler, Rollback-Snapshot/-Restore, weitere Deliveries und Modifikatoren. Nicht im Umfang: Netzwerk, KI, Menüs, Audio, Asset-Pipelines, Spielstände, Animation, Zauber-Skriptsprachen. Keine Abstraktion, die nicht der „Entwirf für"-Liste dient oder mindestens zweimal verwendet wird. Keine TODO-Stubs für Kernfunktionalität.

**Simulations-Invarianten**

Die neun Invarianten stehen unverändert auf der Seite [[simulationsmodell|Simulationsmodell]]; sie sind Teil dieser Spezifikation.

**Anforderungen**

1. Intent und Steuerung: `MageIntent` wie auf der Seite [[intent-steuerung|Intent & Steuerung]], inklusive des Flanken-Protokolls (Latch-Zähler im `InputBuffer`, Ableitung von `pressed` und Reset im Set Intent).
2. Zauber: Blueprint-Registry nach `SpellId`; Deliveries `Projectile` und `GroundArea` (Zielpunkt: Strahl-Boden-Schnitt, geklemmt auf Maximalreichweite; Fuse in Ticks); Modifikatoren `Spread` (on cast), `Homing` (per tick, Zielverlust ohne LOS), `Split` (on impact); Effekte `Damage`, `AreaDamage`, `Slow`; drei Zauber: Bolt, Homing-Bolt, Feuerball, dazu der Fragment-Blueprint; geteilt werden der Modifikator `Spread` und der Effekt `Damage`.
3. Kampf und Physik: Health, Mana mit Regeneration, Cooldowns pro Slot; kinematischer Magier über Avians `MoveAndSlide`; Projektile simulationsbewegt mit Shape-Cast pro Tick; Hindernisse blockieren Projektile, Homing-Sichtlinien und Flächenschaden; Observer `SpellImpact`, `DamageApplied`, `MageDied` als die einen Codepfade; toter Magier behält seine Entity (`Dead`-Marker), Respawn nach 180 Ticks; jede ergebnisrelevante Reihenfolge nach `SimId`.
4. Arena und Präsentation: Greybox-Arena (Boden, Wände, Hindernisse unterschiedlicher Höhe); Third-Person-Orbitkamera, `aim_dir` aus der Kamera; Low-Res-Render-Target mit Nearest-Upscale, Auflösung als Einstellung; Transform-Interpolation zwischen fixen Ticks; HUD mit HP/Mana beider Magier und Slot-Cooldowns.

**Vorgehen**

1. Erst die Dokumentation: Dieser Vault ersetzt das ursprünglich geforderte `DESIGN.md`. Module und Plugins, Set-Kette, Zauber-Datenmodell und alle Entscheidungen mit verworfener Alternative stehen hier.
2. Dann `Cargo.toml`, dann Modul für Modul implementieren, kompilierend und mit sauberem Clippy nach jedem Modul.
3. Abschluss: [[erweiterungspunkte|Erweiterungspunkte]] pflegen, Implementation-Logs je Milestone ergänzen.

**Abnahmekriterien**

- `cargo check`, `cargo clippy -- -D warnings` und `cargo test` laufen sauber.
- `cargo run` öffnet eine spielbare Arena: WASD, Maus-Zielen, Slots auf Tasten 1/2/3; der Dummy nimmt Schaden, stirbt, respawnt.
- Headless-Integrationstest: App nur aus Simulations-Plugins; `MageIntent`, der den Bolt auf den Dummy wirkt; eine feste Anzahl Ticks, ohne Wallclock getrieben (Hilfsfunktion, siehe [[headless-integrationstest|Headless-Integrationstest]]); fester Seed und kurzer Abstand, damit die Streuung den Treffer nicht verhindert; Assertion: Dummy hat HP verloren, Zaubernder hat Mana ausgegeben.
- Determinismustest: dieselbe Intent-Sequenz zweimal vom selben Seed; beide Läufe werden serialisiert und Byte-für-Byte verglichen; gültig, weil `Spread` den geseedeten RNG verbraucht.
- README: Steuerung und Test-Befehle.
