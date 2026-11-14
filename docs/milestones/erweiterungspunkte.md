---
title: "Erweiterungspunkte — KI, Netzwerk, Rollback, neue Zauber-Teile"
---
# Erweiterungspunkte — KI, Netzwerk, Rollback, neue Zauber-Teile

**KI-Controller:** Schnittstelle ist `MageIntent`. Ein AI-Plugin produziert Intents (leerer Dummy-Intent ist der Spezialfall „tue nichts"); die Simulation bleibt unverändert, weil sie Regeln ohnehin caster-unabhängig durchsetzt.

**Netzwerk:** `MageIntent` ist serde-serialisierbar und enthält keine Entity-Referenzen. Pro Tick wird der Intent ausgetauscht; `PlayerId` identifiziert Magier auf dem Draht, `SpellId` identifiziert Blueprints. Für Rollback-PvP gilt die Einschränkung aus [[determinismus|Determinismus]].

**Rollback:** Zu snapshotten sind: alle Simulations-Components (`SimId`, Position und Richtung von Magiern und Zaubern, Health, Mana, Cooldowns, `IntentForTick`, Zustände laufender Zauber inklusive Fuse und Lebensdauer, Slow-Restdauer, `Dead` und Respawn-Timer) und die Ressourcen mit Zustand (RNG, `SimId`-Zähler; Balance ist konstant und bleibt außen vor). Dazu kommt eine Intent-Historie pro Tick, aus der das Set Intent bei der Neuberechnung liest. Explizit nicht gesnapshottet: statische Arena-Kollider und Avian-Interna, weil die Simulation Avian nur abfragt ([[adr-0003-kinematik-statt-dynamischer-rigidbodies|ADR-0003]], [[adr-0007-shape-cast-pro-tick-fuer-projektile|ADR-0007]]). Restore heißt: Simulations-Entities entfernen, Zustand aus dem Snapshot wiederherstellen, Bindung `PlayerId` → Entity neu aufbauen, Avians Query-Struktur einmal auffrischen, Ticks ab dort neu ausführen. Weil jede Reihenfolge an der `SimId` hängt, dürfen die Entities dabei neue Indizes bekommen ([[adr-0013-simid-statt-entity-index-fuer-jede|ADR-0013]]).

**Neues Delivery:** Enum-Variante in `Delivery`, ein System im Set Bewegung, eins im Set Treffer, eine Blueprint-Zeile, fertig. Beispiel-Delivery später: `Beam` (Soforttreffer entlang aim_dir per Raycast).
**Neuer Modifikator:** Component plus genau ein Lifecycle-System (on cast, per tick, on impact, on expire) und eine Enum-Variante in der Registry. Bestehende Zauber unberührt.
**Neuer Effekt:** Enum-Variante plus ein Handler im einzigen Impact-Pfad (`SpellImpact` → Effekte → `DamageApplied`). Es gibt keinen zweiten Schadenspfad, in dem man ihn vergessen könnte.
