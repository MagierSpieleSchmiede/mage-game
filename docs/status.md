---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-14, B-003 (Branch `dev-danielb`):** `mage_sim` hat `TICKS_PER_SECOND` (60), `MageSimPlugin` (stellt den Fixed-Takt ein), `scenario::Scenario` mit Builder (Teams, Spieler pro Team, Seed; 2 bis 32 Teams, kein leeres Team) und `headless::{headless_app, step_ticks}`. Tests in `crates/mage_sim/tests/headless.rs`. Die Bevy-Migrationsleitfäden 0.18→0.20 sind gelesen; für die Simulation wichtig: `Resource` ist jetzt ein Subtrait von `Component` (ein Typ kann nicht beides ableiten), Observer heißen `On<Add<A>>` statt `On<Add, A>`, Lifecycle-Event `Replace` heißt `Discard`. ADR-0015 ist angenommen.

## Nächste Aufgabe

**B-004 — IDs: `SimId`-Zähler, `PlayerId`, `TeamId`, `SpellId`; das Szenario spawnt Magier mit Teams.**

- **Erst klären:** nichts; die nötigen ADRs (0006, 0013, 0015) sind angenommen.
- **Dafür lesen:** die Zeile B-004 im [[backlog|Backlog]]; [[serialisierung-ids|Serialisierung & IDs]]; in [[teams|Teams]] den Abschnitt „Was gilt“; in [[simulationsmodell|Simulationsmodell]] die Invarianten 5 und 6. Code: `crates/mage_sim/src/scenario.rs` und `headless.rs`.
- **Tests zuerst:** `SimId` fortlaufend und eindeutig; ein 2-gegen-2-Szenario hat vier Magier in zwei Teams.

## Hinweise für die nächste Sitzung

- `headless_app` macht schon ein Update (Uhr starten); wer beim Bauen der App spawnt, muss das in `Startup` oder vor diesem Update tun, sonst existieren die Magier erst nach dem ersten Tick.
- IDs sollen serde-serialisierbar sein (ADR-0006); `serde` wäre dann die erste neue Abhängigkeit: Plattformen prüfen und im Commit begründen (`AGENTS.md`).
- Solange nur Daniel arbeitet: auf `dev-danielb` committen, auf sein Wort direkt nach `master` übernehmen.
