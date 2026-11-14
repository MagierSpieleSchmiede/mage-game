---
title: "ADR-0021 — Animation ist reine Präsentation; Fähigkeiten laufen nach Ticks"
---
# ADR-0021 — Animation ist reine Präsentation; Fähigkeiten laufen nach Ticks

**Status:** vorgeschlagen

**Kontext:** In vielen Spielen bestimmt die Animation, wann ein Zauber tatsächlich losgeht (Animations-Events, Root Motion). Dann verzögert oder blockiert eine lange oder hängende Animation die Fähigkeit, der Server müsste Animationen simulieren, und das Spielgefühl hängt an Grafikdaten.

**Entscheidung:** Ob und wann eine Fähigkeit wirkt, entscheidet allein die Simulation nach Ticks (Cooldown, optional eine Wirkzeit in Ticks im Blueprint). Animationen folgen diesem Zustand: Sie werden beim Cast gestartet, in ihrer Geschwindigkeit an die Wirkzeit angepasst und jederzeit abgebrochen oder überblendet, wenn die Simulation etwas Neues tut. Keine Animations-Events lösen Gameplay aus, keine Root Motion bewegt Simulations-Entities.

**Verworfene Alternative:** Animationsgetriebene Fähigkeiten (Event im Clip löst den Zauber aus). Verworfen, weil sie Eingaben verschluckt, auf Server und im headless RL-Training nicht läuft und den Determinismus an Grafikdaten bindet.

**Konsequenzen:** Ein Test stellt sicher, dass ein Cast im selben Tick wirkt, egal ob eine Animation läuft. Animationen müssen kurz und unterbrechbar gestaltet werden ([[einstieg-gestalten|Gestalten]]).
