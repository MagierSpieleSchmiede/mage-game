---
title: "ADR-0006 — serde statt bevy_reflect für Intent und Blueprints"
---
# ADR-0006 — serde statt bevy_reflect für Intent und Blueprints

**Status:** angenommen

**Kontext:** Rollback-Snapshots und Netcode-Pakete brauchen ein serialisierbares Datenmodell.

**Entscheidung:** serde mit binärem Format für `MageIntent`, Blueprints und Simulations-Snapshots.

**Verworfene Alternative:** `bevy_reflect`. Verworfen, weil Reflektion langsamer ist, Versionsstabilität schwerer zu garantieren ist und die Typen ohnehin eigenes, versionsstabilisiertes Datenmodell sein wollen.

**Konsequenzen:** Eine `SerializableWorld`-Hilfe, die registrierte Simulationskomponenten einsammelt; Enum-Diskriminanten werden explizit nummeriert ([[p-004-enum-diskriminanten-stabilitaet-fuer-netcode|P-004]]).
