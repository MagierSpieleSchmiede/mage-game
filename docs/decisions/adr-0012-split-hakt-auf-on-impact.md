---
title: "ADR-0012 — Split hakt auf on impact statt on expire"
---
# ADR-0012 — Split hakt auf on impact statt on expire

**Status:** angenommen

**Kontext:** Split muss sich für genau einen Lifecycle-Punkt entscheiden.

**Entscheidung:** on impact. Fragmente entstehen am Auftreffpunkt, wo auch `AreaDamage` wirkt; der Feuerball-Flow (Impact → Explosion → Fragmente) bleibt ein einziger Pfad.

**Verworfene Alternative:** on expire. Verworfen, weil ein abgelaufener, ins Leere geflogener Zauber an beliebiger Stelle fragmentieren würde und der Feuerball ohnehin fast immer aufprallt.

**Konsequenzen:** Fuse-Ende des GroundArea-Delivery zählt als Impact, damit Split in beiden Wegen gleich funktioniert.
