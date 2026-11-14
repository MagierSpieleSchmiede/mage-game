---
title: "Serialisierung & IDs — serde, PlayerId, SpellId"
---
# Serialisierung & IDs — serde, PlayerId, SpellId

`MageIntent`, `SpellBlueprint`, `Delivery`, `ModifierKind`, `EffectKind`, die IDs und alle Balance-Ressourcen leiten `Serialize`/`Deserialize` ab (eingeführt mit dem ersten Snapshot, B-013) ([[adr-0006-serde-statt-bevy-reflect-fuer-intent|ADR-0006]]). Für Snapshots und spätere Netcode-Pakete bedeutet das: schnelles binäres Serialisieren ohne Reflektion, und ein Objekt, das man Byte-für-Byte vergleichen kann.

Die IDs stehen in `crates/mage_sim/src/ids.rs`: `PlayerId` (u32, Platz für tausende Spieler), `TeamId` (u8, [[teams|Teams]]), `SpellId` (u16). `SimId` (u64) wird beim Spawn jeder Simulations-Entity vom `SimIdCounter` vergeben, der selbst Simulationszustand ist; 64 Bit, weil in der großen Welt ein lange laufender Server ständig Entities erzeugt und hereinlädt ([[welt|Welt]]). Sie bestimmt jede ergebnisrelevante Reihenfolge ([[adr-0013-simid-statt-entity-index-fuer-jede|ADR-0013]]). Entities existieren nur im Arbeitsspeicher; jede Zuordnung (`PlayerId` → Mage-Entity) wird beim Spawn aufgebaut und gehört in keinen Snapshot-Payload. Für Netcode-Stabilität bekommen Enum-Varianten explizite, nummerierte Diskriminanten ([[p-004-enum-diskriminanten-stabilitaet-fuer-netcode|P-004]]).
