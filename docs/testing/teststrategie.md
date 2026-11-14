---
title: "Teststrategie — drei Ebenen"
---
# Teststrategie — drei Ebenen

1. **Unit:** Flanken-Ableitung (Latch-Konsum, Reset), Cooldown-Dekrement, Blueprint-Lookup. Reine Funktionen, keine App.
2. **Headless-Integration:** App aus Simulations-Plugins, ohne Fenster und Renderer (Invariante 7). Der Test schreibt Intents und liest Components.
3. **Determinismus:** zwei Läufe, ein Seed, Byte-Vergleich des serialisierten Simulationszustands.
