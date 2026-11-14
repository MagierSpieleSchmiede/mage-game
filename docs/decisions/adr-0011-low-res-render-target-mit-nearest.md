---
title: "ADR-0011 — Low-Res-Render-Target mit Nearest-Upscale"
---
# ADR-0011 — Low-Res-Render-Target mit Nearest-Upscale

**Kontext:** Stilisierter Retro-Look, später einstellbar.
**Entscheidung:** Szene rendert in ein verkleinertes Render-Target, Upscale per Nearest-Filterung; die Auflösung ist eine Einstellung im Render-Plugin.
**Verworfene Alternative:** Native Auflösung mit Post-Processing-Filter. Verworfen, weil teurer und der Pixel-Look weniger präzise steuerbar.
**Konsequenzen:** HUD rendert in voller Auflösung; Simulationszustand ist davon unberührt.
