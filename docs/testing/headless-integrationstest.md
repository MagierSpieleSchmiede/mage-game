---
title: "Headless-Integrationstest — wie Ticks ohne Wallclock getrieben werden"
---
# Headless-Integrationstest — wie Ticks ohne Wallclock getrieben werden

`app.update()` akkumuliert echte Zeit; die Tickzahl hinge von der Uhr ab. Deshalb setzt der Test `TimeUpdateStrategy::ManualDuration` auf genau eine Tick-Dauer: Jedes `app.update()` führt dann exakt einen fixen Tick aus, mit allen Fixed-Schedules und damit auch mit Avians Schritt. Die Hilfe `step_ticks(&mut app, n)` ruft `app.update()` n-mal auf und lebt im Test-Support-Code, nicht in den Simulations-Plugins. Den `FixedUpdate`-Schedule von Hand auszuführen ist keine Alternative, weil dabei die Zeit-Ressourcen nicht fortgeschrieben werden. Der Test: Dummy per `PlayerId` suchen und 5 m vor den Zaubernden stellen, festen Seed setzen, Bolt-Intent schreiben, 60 Ticks steppen, HP-Differenz und Mana-Differenz assertieren.
