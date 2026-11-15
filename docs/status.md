---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-15, B-007 und B-008 (Branch `dev-danielb`):**

- **B-007 Attribute:** `mage_sim::attributes`, `balance::Balance`, `unit::Unit` (Kern-Marker; `Mage` verlangt ihn), `update_attributes` im Set Werte. ADR-0016 angenommen.
- **B-008 Zufall:** `mage_sim::rng::SimRng` (eigener Generator, Xoshiro256\*\* mit SplitMix64, aus dem Seed des Szenarios); `crates/mage_sim/clippy.toml` verbietet andere Zufalls- und Zeitquellen sowie `std::collections::HashMap`/`HashSet`.

## Nächste Aufgabe

**B-009 — Einheiten-Kern: Leben, Mana in Tausendsteln, Regeneration, Cooldowns, Bewegung per `MoveAndSlide` mit effektivem Tempo; für alle Einheiten.**

- **Erst klären:** nichts; alle nötigen ADRs (0003, 0008, 0016, 0031) sind angenommen.
- **Dafür lesen:** die Zeile B-009 im [[backlog|Backlog]]; [[kampf-physik|Kampf & Physik]] (Abschnitte Magier und Bewegung); in [[einheiten|Einheiten]] die Tabelle „Kern und Zusätze“. `MoveAndSlide` in Avian 0.8 im Quellcode prüfen (`~/.cargo/registry/src/*/avian3d-0.8.0/src/character_controller/` oder per `grep -rn "MoveAndSlide"`). Code: `crates/mage_sim/src/{unit,attributes,balance,intent,schedule}.rs`.
- **Tests zuerst:** Regeneration vor Ausgaben im selben Tick; Cooldown N heißt frühestens N Ticks später; ein Tempo-Modifikator ändert den Weg pro Tick; Leben und Mana nie über dem effektiven Maximum.

## Hinweise für die nächste Sitzung

- Mana und Leben in Tausendsteln (`i64`), Raten aus `EffectiveAttributes` pro Sekunde, umgerechnet über `TICKS_PER_SECOND`; ganzzahlig, damit 5 Mana/s bei 60 Ticks aufgeht (5000 / 60 ist nicht ganzzahlig: Rest übertragen oder Rate pro Tick in der Balance festlegen und begründen).
- Neue Systeme fragen Kern-Components ab (`With<Unit>`), nicht `With<Mage>`.
- `HashMap`/`HashSet` aus `std` sind in `mage_sim` verboten (Clippy); `BTreeMap` nehmen.
- Frames mit 0 oder mehreren Ticks: `TimeUpdateStrategy::FixedTimesteps(n)` für ein `app.update()`, danach wieder 1 (siehe `tests/intent.rs`).
- Wer eine Quelldatei mit `mv` zurückspielt, muss sie danach `touch`en, sonst baut Cargo nicht neu.
