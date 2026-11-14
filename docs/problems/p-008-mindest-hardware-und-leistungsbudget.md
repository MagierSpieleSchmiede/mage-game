---
title: "P-008 — Mindest-Hardware und Leistungsbudget"
---
# P-008 — Mindest-Hardware und Leistungsbudget

Das Spiel soll auch auf alten Rechnern laufen. Dafür braucht es einen konkreten **Referenzrechner** (zum Beispiel ein Notebook von etwa 2015 mit integrierter Grafik, Dual-Core, 4 GB RAM) und daraus abgeleitete Budgets: Simulations-Tick, Bildzeit bei welcher Fenstergröße, Speicher, Bandbreite pro Client. Offen ist auch, welche Grafik-Schnittstelle dort verfügbar ist (Vulkan, DirectX 12, Metal oder nur OpenGL/GLES über den GL-Pfad von wgpu) und ob der Low-Res-Look mit dem GL-Pfad gleich aussieht.

Nächster Schritt: in M4 einen Referenzrechner festlegen (am besten ein echtes Gerät im Team), die Platzhalter-Budgets auf [[benchmarks|Benchmarks]] durch Messungen ersetzen und als ADR festhalten.
