---
title: "Serialisierung & IDs — serde, PlayerId, SpellId"
---
# Serialisierung & IDs — serde, PlayerId, SpellId

`MageIntent`, `SpellBlueprint`, `Delivery`, `ModifierKind`, `EffectKind` und alle Balance-Ressourcen leiten `Serialize`/`Deserialize` ab ([[adr-0006-serde-statt-bevy-reflect-fuer-intent|ADR-0006]]). Für Snapshots und spätere Netcode-Pakete bedeutet das: schnelles binäres Serialisieren ohne Reflektion, und ein Objekt, das man Byte-für-Byte vergleichen kann.

`PlayerId` und `SpellId` sind stabile IDs (u16). `SimId` (u32) wird beim Spawn jeder Simulations-Entity aus einem Zähler vergeben, der selbst Simulationszustand ist; sie bestimmt jede ergebnisrelevante Reihenfolge ([[adr-0013-simid-statt-entity-index-fuer-jede|ADR-0013]]). Entities existieren nur im Arbeitsspeicher; jede Zuordnung (`PlayerId` → Mage-Entity) wird beim Spawn aufgebaut und gehört in keinen Snapshot-Payload. Für Netcode-Stabilität bekommen Enum-Varianten explizite, nummerierte Diskriminanten ([[p-004-enum-diskriminanten-stabilitaet-fuer-netcode|P-004]]).
