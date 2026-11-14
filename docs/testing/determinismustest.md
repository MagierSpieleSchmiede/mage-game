---
title: "Determinismustest — zwei Läufe, ein Vergleich"
---
# Determinismustest — zwei Läufe, ein Vergleich

Beide Läufe starten mit demselben Seed und derselben Intent-Sequenz (auch die Intents werden deterministisch eingespielt). Danach wird der simulierte Zustand beider Welten über die serde-Hilfe serialisiert und Byte-für-Byte verglichen. Der Test scheitert damit nicht nur bei echtem Zufall, sondern bei jeder Iterationsreihenfolge, die von Entity-Spawn-Reihenfolge oder HashMap-Layout abhängt. Der `Spread`-Modifikator sorgt dafür, dass der RNG im Prüfgebiet überhaupt beteiligt ist.
