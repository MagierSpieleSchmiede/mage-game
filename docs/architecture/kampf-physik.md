---
title: "Kampf & Physik — Leben, Mana, kinematische Magier, Shape-Casts, Observer"
---
# Kampf & Physik — Leben, Mana, kinematische Magier, Shape-Casts, Observer

**Magier:** Health und Mana sind Components mit Regeneration pro Tick (regeneriert im Set Casting, vor allen Ausgaben des Ticks). Mana wird intern in Tausendsteln gezählt (`u32`), damit Raten wie 5 Mana pro Sekunde bei 60 Ticks ganzzahlig bleiben; Blueprint-Kosten stehen in ganzen Mana. Cooldowns pro Slot sind Tick-Zähler-Components: Beim Cast wird der Zähler auf `cooldown_ticks` gesetzt, ab dem Folgetick zu Beginn des Sets Casting dekrementiert, und bei 0 ist der Slot frei. Ein Cooldown von N heißt also: frühestens N Ticks nach dem Cast wieder möglich. Alle Balance-Konstanten (maximale Werte, Regenerationsrate, Zauber-Kosten, Respawn-Verzögerung) liegen in einer zentralen Balance-Ressource, niemals als magische Zahlen in Systemen.

**Bewegung:** Der Magier ist kinematisch und wird über Avians `MoveAndSlide` bewegt (seit Avian 0.6 vorhanden; [[p-001-move-and-slide|P-001]] ist damit erledigt). `MoveAndSlide` ist ein Systemparameter, den das eigene Bewegungssystem im Set Bewegung aufruft; es gibt keinen automatischen Controller, der im Set Physik liefe.

**Projektile:** bewegt von der Simulation, Treffererkennung per Shape-Cast entlang jedes Ticks. Hindernisse blockieren Projektile (Impact am Hindernis) und Sichtlinien (Homing-Verlust). Tunnelling durch dünne Wände ist konstruktiv ausgeschlossen, weil der Cast das komplette Travel-Segment prüft.

**Observer als der eine Codepfad:** Diskrete Events sind Observer: `SpellImpact` (ein Zauber trifft; löst die on-Impact-Effekte aus), `DamageApplied` (HP sinkt, immer über genau einen Handler), `MageDied` (Tod, Respawn-Timer starten). Ausgelöst werden sie ausschließlich im Set Schaden, in nach `SimId` sortierter Reihenfolge ([[adr-0014-impacts-sammeln-dann-sortiert-ausloesen|ADR-0014]]). Alles Kontinuierliche (Bewegung, Cooldowns, Regeneration, Slow-Abklingen, Fuse-Timer) sind gewöhnliche Systeme in der Kette. Es gibt bewusst keinen zweiten Schadenspfad: Niemand mutiert HP direkt, jede Schadensquelle läuft über `DamageApplied`.

**Respawn:** Ein toter Magier behält seine Entity und bekommt einen `Dead`-Marker; Bewegung, Casting und Treffer ignorieren ihn. Nach einer festen Tick-Anzahl (Platzhalter 180) wird er an seinen Startpunkt gesetzt, bekommt volle Werte und verliert den Marker. So bleiben `PlayerId`-Bindung, HUD und Kamera gültig. Kein Sekunden-Timer, keine Frame-Zeit.
