---
title: "ADR-0024 — Plattformneutralität: keine Entscheidung, die eine Zielplattform ausschließt"
---
# ADR-0024 — Plattformneutralität: keine Entscheidung, die eine Zielplattform ausschließt

**Status:** angenommen (2026-11-14)

**Kontext:** Das Spiel soll auf Windows, Linux und macOS laufen, außerdem auf Android und iOS, und wenn es ohne größere Probleme geht, im Browser (WebAssembly). Jede dieser Plattformen hat Einschränkungen: keine Threads und kein Dateisystem im Browser, kein JIT auf iOS, Touch statt Maus auf Mobilgeräten, schwächere GPUs.

**Entscheidung:** Zielstufen: **Stufe 1** Windows, Linux, macOS (muss immer bauen und laufen). **Stufe 2** Android, iOS (muss immer bauen; laufen ab Milestone 9). **Stufe 3** Web (`mage_sim` muss immer für `wasm32-unknown-unknown` bauen; ein Web-Client, wenn er ohne größere Probleme machbar ist). Für jede Abhängigkeit und jede Technik gilt: Sie darf keine Stufe-1- oder Stufe-2-Plattform ausschließen. Die Simulation nutzt keine Threads, kein Dateisystem, keine Systemzeit und keine nativen Bibliotheken. Rendering-Grundlook bleibt in den Grenzen von WebGL2 und mobilen GPUs.

**Verworfene Alternative:** Erst Desktop, später portieren. Verworfen, weil Netzwerk-Transport, Add-on-Laufzeit, Eingabe und Rendering sonst Entscheidungen treffen, die Mobile oder Web unmöglich machen.

**Konsequenzen:** Bei jeder neuen Abhängigkeit wird die Plattformunterstützung geprüft. CI baut für alle Stufen ([[p-014-mobile-und-web-builds|P-014]]). Details: [[plattformen|Plattformen]].
