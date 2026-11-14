---
title: "Determinismus — was garantiert wird, und auf welcher Ebene"
---
# Determinismus — was garantiert wird, und auf welcher Ebene

Garantiert ist bit-identische Reproduktion **auf derselben Plattform- und Compiler-Konfiguration**: gleicher Startzustand, gleiche Intent-Sequenz, gleicher Seed ⇒ identischer serialisierter Endzustand. Dafür sorgen: eine einzige geseedete RNG-Ressource, alle Timer in Ticks, keine echte oder virtuelle Frame-Zeit in Simulationscode, definierte Iterationsreihenfolge, wo Reihenfolge das Ergebnis beeinflusst, und Schaden/Mana/Tod über exakt einen Pfad.

**Nicht** garantiert ist plattformübergreifende Bit-Exaktheit: `f32`-Operationen und Avian-Queries dürfen auf verschiedenen Architekturen abweichen. Für echtes Netzwerk-PvP heißt das: Plattformen locken oder auf Fixed-Point wechseln ([[p-003-cross-plattform-determinismus-fuer-echtes-pvp|P-003]]). Der Prototyp trifft diese Entscheidung bewusst nicht vor.

Der Determinismustest hat nur Zähne, weil mindestens ein System den RNG verbraucht: `Spread` streut den Bolt und macht den Zufall prüfbar ([[adr-0010-spread-modifikator-als-der-rng-verbraucher|ADR-0010]]).
