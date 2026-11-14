---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-15, B-005 (Branch `dev-danielb`):** Die Tick-Kette steht: `mage_sim::schedule::SimSet` (Intent, Values, Casting, Movement, Physics, Hits, Damage, Visibility, Cleanup), verkettet in `FixedUpdate`; Avian läuft im Set Physics. Ein Shape-Cast im Set Hits sieht Positionen desselben Ticks; mit Avians Standard-Schedule tat er das nicht (P-005 erledigt). `headless_app` hat jetzt `TaskPoolPlugin`, `TransformPlugin`, `finish()`/`cleanup()`. Tests in `crates/mage_sim/tests/tick_chain.rs`. Davor angenommen: ADR-0030 (Fähigkeitenleiste) und ADR-0031 (Einheiten).

## Nächste Aufgabe

**B-006 — `MageIntent`, `InputBuffer` nach `SimId` der gesteuerten Einheit, `Controller` (Spieler oder KI), Flanken-Protokoll, `IntentForTick`; Slots bis `MAX_SLOTS`.**

- **Erst klären:** nichts; ADR-0004, ADR-0030 und ADR-0031 sind angenommen.
- **Dafür lesen:** die Zeile B-006 im [[backlog|Backlog]]; [[intent-steuerung|Intent & Steuerung]]; in [[einheiten|Einheiten]] die Tabelle „Kern und Zusätze“ (`Controller`); in [[faehigkeiten|Fähigkeiten & Belegung]] den Abschnitt „Regeln“ (`MAX_SLOTS`, Platzhalter 12). Code: `crates/mage_sim/src/{lib,schedule,mage}.rs`.
- **Tests zuerst:** Latch-Garantien bei 0, 1 und mehreren Ticks pro Frame (ein Druck geht nie verloren, wird nie doppelt gezählt); `just_released`; ein Intent wirkt nur auf die Einheit, für die er geschrieben wurde.

## Hinweise für die nächste Sitzung

- Frames mit 0 oder mehreren Ticks lassen sich mit `TimeUpdateStrategy::FixedTimesteps(0)` bzw. `(n)` für ein einzelnes `app.update()` nachstellen; `step_ticks` setzt das nicht zurück, also im Test danach wieder auf 1 setzen.
- `MageIntent` ohne serde, bis der Snapshot (B-013) es braucht; `aim_dir` und `move_axis` werden schon hier geklemmt bzw. normalisiert, NaN wird verworfen ([[sicherheit|Sicherheit]]).
- Bevy-Fehler „Resource does not exist“ zeigen Systemnamen nur mit `cargo test --features bevy/debug`.
- Neue Systeme fragen Kern-Components ab (`SimId`, `TeamId`), nicht `With<Mage>`.
