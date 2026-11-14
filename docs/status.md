---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-15, B-006 (Branch `dev-danielb`):** `mage_sim::intent` mit `MAX_SLOTS` (12), `MageIntent`, `IntentForTick`, `Controller`, `InputBuffer` (nach `SimId`, Latch-Zähler) und `freeze_intents` im Set Intent, inklusive Bereinigung ungültiger Eingaben. Magier haben `Controller::Player`. 11 Tests in `crates/mage_sim/tests/intent.rs`.

## Nächste Aufgabe

**B-008 — Geseedete RNG-Ressource.** (B-007 Attribute käme vorher, baut aber auf [[adr-0016-attribute-mit-modifikatoren|ADR-0016]], die noch vorgeschlagen ist; Daniel ist gefragt.)

- **Erst klären:** nichts; [[adr-0009-eine-geseedete-rng-ressource-statt-verstreuter|ADR-0009]] ist angenommen.
- **Dafür lesen:** die Zeile B-008 im [[backlog|Backlog]]; ADR-0009; [[determinismus|Determinismus]]. Code: `crates/mage_sim/src/{lib,scenario}.rs` (der Seed steht im `Scenario`).
- **Tests zuerst:** gleicher Seed → gleiche Folge, anderer Seed → andere Folge; eine Clippy-Regel (`disallowed_methods`/`disallowed_types` in `clippy.toml`) verbietet andere Zufallsquellen in `mage_sim`, nachgewiesen durch einen absichtlichen Verstoß, der Clippy rot macht.

## Hinweise für die nächste Sitzung

- RNG: Eine neue Abhängigkeit (etwa `rand_chacha`) braucht eine Begründung und Plattformprüfung; ein kleiner eigener Generator (z. B. SplitMix64 oder Xoshiro, ein Dutzend Zeilen, ganzzahlig) wäre plattformübergreifend bit-gleich und ohne Abhängigkeit. Entscheidung im Commit begründen.
- Frames mit 0 oder mehreren Ticks: `TimeUpdateStrategy::FixedTimesteps(n)` für ein `app.update()`, danach wieder 1 (siehe `tests/intent.rs`, `frame`).
- Wer eine Quelldatei mit `mv` zurückspielt, muss sie danach `touch`en, sonst baut Cargo nicht neu.
- Neue Systeme fragen Kern-Components ab (`SimId`, `TeamId`), nicht `With<Mage>`.
