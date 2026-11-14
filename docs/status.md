---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-15, B-007 (Branch `dev-danielb`):** Attribute nach ADR-0016: `mage_sim::attributes`, `mage_sim::balance::Balance` (Platzhalter-Basiswerte, Grenzen, `max_stacks`), `mage_sim::unit::Unit` als Kern-Marker der Einheiten; `update_attributes` im Set Werte. 10 Tests in `crates/mage_sim/tests/attributes.rs`. Neu angenommen: ADR-0016.

## Nächste Aufgabe

**B-008 — Geseedete RNG-Ressource, mit eigenem Generator** (Daniel, 2026-11-15: eigener Generator statt Bibliothek, wenn er verlässlich dieselben Zahlen liefert).

- **Erst klären:** nichts; [[adr-0009-eine-geseedete-rng-ressource-statt-verstreuter|ADR-0009]] ist angenommen.
- **Dafür lesen:** die Zeile B-008 im [[backlog|Backlog]]; ADR-0009; [[determinismus|Determinismus]]. Code: `crates/mage_sim/src/{lib,scenario}.rs` (der Seed steht im `Scenario`).
- **Tests zuerst:** gleicher Seed → gleiche Folge; anderer Seed → andere Folge; feste Referenzwerte für einen Seed (damit eine Änderung am Generator auffällt und dieselben Zahlen auf jeder Plattform belegt sind); eine Clippy-Regel in `clippy.toml` verbietet andere Zufallsquellen in `mage_sim`, nachgewiesen durch einen absichtlichen Verstoß.

## Hinweise für die nächste Sitzung

- RNG: eigener Generator, nur ganzzahlige Operationen mit festgelegtem Überlauf (`wrapping_*`), damit er auf jeder Plattform bit-gleich ist (z. B. SplitMix64 zum Seeden, Xoshiro256** für die Folge).
- Frames mit 0 oder mehreren Ticks: `TimeUpdateStrategy::FixedTimesteps(n)` für ein `app.update()`, danach wieder 1 (siehe `tests/intent.rs`, `frame`).
- Wer eine Quelldatei mit `mv` zurückspielt, muss sie danach `touch`en, sonst baut Cargo nicht neu.
- Neue Systeme fragen Kern-Components ab (`SimId`, `TeamId`), nicht `With<Mage>`.
