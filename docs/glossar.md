---
title: "Glossar"
---
# Glossar

| Begriff | Bedeutung |
| --- | --- |
| MageIntent | serialisierbare Per-Tick-Steuerungsdaten eines Magiers (Bewegung, Zielrichtung, Knopfzustände) |
| InputBuffer | Ressource, in der lokale Intent-Produzenten zwischen den Ticks Eingaben sammeln; das Set Intent friert sie zu einem MageIntent ein |
| Tick | ein fixer Simulationsschritt; alle Dauern sind Tick-Zähler |
| Latch | Zähler für Druckereignisse zwischen zwei Ticks im InputBuffer; die Simulation konsumiert und resettet ihn |
| Flanke | Übergang gerade gedrückt/gerade losgelassen, abgeleitet aus Latch und Vor-Tick-Zustand |
| Blueprint | serialisierbare Zauber-Konfiguration in der Registry, Schlüssel `SpellId` |
| Delivery | wie ein Zauber zum Ziel kommt: `Projectile` (Flugbahn) oder `GroundArea` (Bodenpunkt mit Fuse) |
| Modifikator | Component an der Zauber-Entity mit einem eigenen System und genau einem Lifecycle-Haken |
| Effekt | reine Daten im on-Impact-Pfad: `Damage`, `AreaDamage`, `Slow` |
| LOS | Line of Sight; Sichtlinie, die Arena-Kollider blockieren (Verbraucher: Homing) |
| Fuse | Tick-Zähler eines GroundArea-Zaubers bis zum Impact |
| PlayerId / SpellId | stabile IDs für Magier und Blueprints; ersetzen Entities in jedem serialisierten Datum |
| SimId | fortlaufende ID jeder Simulations-Entity; bestimmt jede ergebnisrelevante Reihenfolge und liegt im Snapshot |
| Headless | Simulations-Plugins ohne Fenster und Renderer, Normalfall der Tests |
| Greybox | untexturierte Platzhaltergeometrie der Arena |
| Rollback | Netcode-Ansatz: Snapshots des Simulationszustands, Zurückspulen und Neuausführen ab einem Tick |
| ADR | Architecture Decision Record; nummerierte Entscheidung mit verworfener Alternative |
| P-Log | Problem-Log; nummerierte offene Fragen mit nächstem Schritt |
