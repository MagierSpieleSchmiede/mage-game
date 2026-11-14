---
title: "Plattformen — Desktop, Mobile und, wenn machbar, Web"
---
# Plattformen — Desktop, Mobile und, wenn machbar, Web

Mage Game soll auf Windows, Linux und macOS laufen, auf Android und iOS, und im Browser, wenn das ohne größere Probleme geht. Keine Entscheidung darf eine dieser Plattformen ausschließen ([[adr-0024-plattformneutralitaet|ADR-0024]], vorgeschlagen).

## Stufen

| Stufe | Plattformen | Anspruch |
| --- | --- | --- |
| 1 | Windows, Linux, macOS | baut und läuft immer; CI baut und testet |
| 2 | Android, iOS | baut immer (ab M1 `cargo check` der Simulation); spielbar ab M9 |
| 3 | Web (WebAssembly) | `mage_sim` baut immer für `wasm32-unknown-unknown`; ein Web-Client, wenn machbar (M9) |

## Regeln für Code und Abhängigkeiten

- **Simulation (`mage_sim`):** keine Threads, kein Dateisystem, keine Systemzeit, keine nativen C-Bibliotheken, keine Abhängigkeit, die nicht für WebAssembly baut. Avian ohne das Feature `parallel` im Web-Build.
- **Assets** nur über Bevys Asset-Server laden, nie über Dateipfade; Einstellungen und Tastenbelegung über eine kleine Speicher-Schnittstelle (Datei auf Desktop, App-Speicher auf Mobile, Browser-Speicher im Web).
- **Rendering:** der Grundlook (Low-Res-Target, einfache Materialien) muss mit WebGL2 und mobilen GPUs funktionieren. Effekte, die neuere Schnittstellen brauchen, sind optional und haben einen Rückfall.
- **Eingabe:** über Aktionen, so dass Touch auf Mobile nur eine weitere Belegung ist ([[eingabegeraete|Eingabegeräte]]).
- **Netzwerk:** Transport muss im Browser funktionieren (WebTransport oder WebSocket), siehe [[p-009-netzwerk-crate-und-transport|P-009]].
- **Add-ons:** Laufzeit muss auf iOS (kein JIT) und im Web laufen, siehe [[p-010-add-on-laufzeit-und-fairness|P-010]].
- **Neue Abhängigkeit:** vor dem Hinzufügen prüfen, ob sie für alle Stufe-1- und Stufe-2-Ziele baut; Ergebnis in den Commit schreiben.
- **Leistung:** alte Rechner und Mobilgeräte bestimmen das Budget ([[benchmarks|Benchmarks]], [[p-008-mindest-hardware-und-leistungsbudget|P-008]]).

## Ziele für CI

| Ziel | Prüfung |
| --- | --- |
| `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`, `aarch64-apple-darwin` | Build, Tests |
| `aarch64-linux-android`, `aarch64-apple-ios` | `cargo check`, später Paket |
| `wasm32-unknown-unknown` | `cargo check -p mage_sim`, später Web-Client |

Ab wann welche Stufe was prüft: [[p-014-mobile-und-web-builds|P-014]].
