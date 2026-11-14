---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-14, B-002 (Branch `dev-danielb`):** CI in `.github/workflows/ci.yml`: fmt, clippy, Tests auf Linux, Windows, macOS; `cargo check -p mage_sim` für wasm32, Android, iOS. Auf allen sieben Jobs grün; ein absichtlicher Clippy-Fehler macht sie rot. Android braucht das Feature `android-check` von `mage_sim`, iOS läuft auf macOS ([[plattformen|Plattformen]]). ADR-0024 ist angenommen. Davor B-001: Workspace `crates/mage_sim` und `crates/mage_client`.

## Nächste Aufgabe

**B-003 — Test-Support: Szenario-Builder (Teams, Spieler, Seed), headless App aus `mage_sim`, `step_ticks`.**

- **Erst klären:** Der Szenario-Builder kennt Teams; dafür sollte Daniel [[adr-0015-teams-statt-fester-zwei-spieler|ADR-0015]] annehmen.
- **Dafür lesen:** die Zeile B-003 im [[backlog|Backlog]]; [[headless-integrationstest|Headless-Integrationstest]]; [[teststrategie|Teststrategie]] (Abschnitt Regeln); in [[teams|Teams]] den Punkt „Szenario“. Das ist der **erste Code mit Bevy-API**: vorher die Migrationsleitfäden Bevy 0.18→0.19 und 0.19→0.20 lesen und `TimeUpdateStrategy::ManualDuration` im Quellcode von Bevy 0.20.1 prüfen (`~/.cargo/registry/src/`).
- **Tests zuerst:** `step_ticks(n)` führt genau n fixe Ticks aus.

## Hinweise für die nächste Sitzung

- CI-Ergebnisse ohne angemeldetes `gh`: `https://api.github.com/repos/MagierSpieleSchmiede/mage-game/actions/runs` (öffentlich).
- Lokal ist `cargo fmt --check` auf Daniels Rechner kaputt (alte rustfmt); die CI prüft das Format. Reparatur: `rustup component remove rustfmt && rustup component add rustfmt`.
- Solange nur Daniel arbeitet: auf `dev-danielb` committen, auf sein Wort direkt nach `master` übernehmen.
