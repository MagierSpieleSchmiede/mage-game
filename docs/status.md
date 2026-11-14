---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-14, B-001 (Branch `dev-danielb`):** Die Planungsphase ist vorbei, Milestone 1 hat begonnen. Das Repo ist ein Workspace mit `crates/mage_sim` (Bevy nur `std`, Avian nur `3d` und `parry-f32`) und `crates/mage_client` (leeres Programm; `cargo run` zeigt nichts). `crates/mage_sim/tests/no_render_dependencies.rs` hält per `cargo tree` fest, dass die Simulation keine Fenster-, Render- oder Audio-Crate erreicht. `cargo check -p mage_sim --target wasm32-unknown-unknown` läuft. ADR-0020 und ADR-0026 sind mit dem Start angenommen.

## Nächste Aufgabe

**B-002 — CI: fmt, clippy, test auf Linux, Windows, macOS; `cargo check -p mage_sim` für wasm32, Android, iOS.**

- **Erst klären:** Daniel nimmt [[adr-0024-plattformneutralitaet|ADR-0024]] (Plattformneutralität) an.
- **Dafür lesen:** die Zeile B-002 im [[backlog|Backlog]]; [[plattformen|Plattformen]] (Abschnitt „Ziele für CI“); [[p-014-mobile-und-web-builds|P-014]]; `.github/workflows/deploy.yml` als Vorbild für Stil und Kommentare.
- **Tests zuerst:** der CI-Lauf selbst; er muss auf allen Zielen grün sein. Dass er rot wird, wenn er soll, einmal mit einem absichtlichen Clippy-Fehler auf dem Branch zeigen.

## Hinweise für die nächste Sitzung

- **rustfmt ist auf Daniels Rechner kaputt:** `cargo fmt --check` meldet „Unrecognized option“, die installierte rustfmt ist das alte, abgekündigte Programm. Das Format von B-001 ist deshalb lokal ungeprüft; die CI aus B-002 prüft es zum ersten Mal. Reparatur: `rustup component remove rustfmt && rustup component add rustfmt`.
- `mage_sim` hat nur reine Rust-Abhängigkeiten; `cargo check` für Android und iOS sollte ohne NDK oder Xcode gehen, braucht aber `rustup target add`.
- Gearbeitet wird auf `dev-danielb`; nach `master` per Pull Request.
