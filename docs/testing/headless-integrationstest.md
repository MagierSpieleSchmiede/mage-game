---
title: "Headless-Integrationstest — wie Ticks ohne Wallclock getrieben werden"
---
# Headless-Integrationstest — wie Ticks ohne Wallclock getrieben werden

`app.update()` akkumuliert sonst echte Zeit; die Tickzahl hinge von der Uhr ab. Deshalb setzt `mage_sim::headless::headless_app(scenario)` die Strategie `TimeUpdateStrategy::FixedTimesteps(1)` (neu in Bevy 0.20): Jedes `app.update()` schiebt die Zeit um genau einen fixen Schritt vor und führt damit genau einen fixen Tick aus, mit allen Fixed-Schedules und damit auch mit Avians Schritt. Das allererste `app.update()` startet nur die Uhr und führt keinen Tick aus; `headless_app` erledigt es, bevor es die App zurückgibt. `step_ticks(&mut app, n)` ruft `app.update()` n-mal auf. Beides liegt im Modul `headless` neben den Simulations-Plugins, nicht in ihnen; Tests, Benchmarks, Server und RL nutzen es gleich. Den `FixedUpdate`-Schedule von Hand auszuführen ist keine Alternative, weil dabei die Zeit-Ressourcen nicht fortgeschrieben werden. Festgehalten in `crates/mage_sim/tests/headless.rs`.

Der Spec-Test (ab B-011): Dummy per `PlayerId` suchen und 5 m vor den Zaubernden stellen, festen Seed setzen, Bolt-Intent schreiben, 60 Ticks steppen, HP-Differenz und Mana-Differenz assertieren.
