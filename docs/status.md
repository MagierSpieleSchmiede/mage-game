---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-14:** Vault entdoppelt: Stand jeder ADR nur noch in [[entscheidungen|Entscheidungen]], Ablauf pro Item nur im [[backlog|Backlog]], Nummernzähler je an einer Stelle ([[konventionen|Konventionen]], Abschnitt „Eine Quelle pro Tatsache“). Branch `preview` veröffentlicht die Webseite zum Probelesen unter `/preview/`. Implementiert ist noch nichts; `cargo run` gibt „Hello, world!“ aus.

## Nächste Aufgabe

**B-001 — Cargo-Workspace mit `mage_sim` und `mage_client`** (Milestone 1, erstes Item im [[backlog|Backlog]]).

- **Erst klären:** Daniel gibt den Start der Implementierung frei und nimmt die vorgeschlagenen ADRs an, auf denen M1 baut, zuerst [[adr-0020-simulation-als-eigene-crate|ADR-0020]] (Workspace) und [[adr-0026-tests-zuerst|ADR-0026]] (Tests zuerst); für die folgenden Items ADR-0015, ADR-0016 und ADR-0024.
- **Dafür lesen:** die Zeile B-001 und den Ablauf oben im [[backlog|Backlog]]; [[adr-0020-simulation-als-eigene-crate|ADR-0020]]; [[projektstruktur|Projektstruktur]] (Abschnitt Crates); [[plattformen|Plattformen]] (Regeln für `mage_sim`); die Migrationsleitfäden Bevy 0.18→0.19 und 0.19→0.20 (Regel in `AGENTS.md`).
- **Tests zuerst:** `cargo test` läuft im Workspace; ein Test stellt sicher, dass `mage_sim` keine Render-Crate im Abhängigkeitsbaum hat.

## Hinweise für die nächste Sitzung

- Der Wechsel auf einen Workspace verändert `Cargo.lock` stark: danach einmal `cargo clean` (`AGENTS.md`, Abschnitt `target/`).
- Ab Beginn der Implementierung wird auf einem eigenen Branch mit Pull Request gearbeitet, nicht mehr direkt auf `master`.
