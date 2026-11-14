---
title: "P-004 — Enum-Diskriminanten-Stabilität für Netcode"
---
# P-004 — Enum-Diskriminanten-Stabilität für Netcode

Sobald `Delivery`, `ModifierKind`, `EffectKind` auf dem Draht landen, sind implizit vergebene Varianten-Nummern ein Protokollbruch. Vor dem Netcode-Meilenstein: explizite `#[repr(u8)]`-Nummerierung und Versionierung der Serialisierungsformate festzurren.
