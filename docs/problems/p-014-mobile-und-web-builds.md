---
title: "P-014 — Mobile- und Web-Builds in CI"
---
# P-014 — Mobile- und Web-Builds in CI

Damit keine Entscheidung eine Plattform still ausschließt ([[adr-0024-plattformneutralitaet|ADR-0024]]), muss CI für alle Stufen bauen: Windows, Linux, macOS, Android, iOS und `wasm32-unknown-unknown`. iOS-Builds brauchen einen macOS-Runner; Android braucht NDK und eine Paketierung; ein Web-Client braucht eine HTML-Hülle und WebGL2/WebGPU. Offen ist, ab wann welche Stufe nur `cargo check`, wann ein echter Build und wann ein Starttest auf einem Gerät oder Emulator läuft.

Stand M1: `cargo check` für `mage_sim` läuft in CI auf allen Zielen (B-002, Eigenheiten auf [[plattformen|Plattformen]]). Nächster Schritt: in M4 vollständige Desktop-Builds; in M9 Mobile-Pakete und, wenn machbar, ein Web-Client.
