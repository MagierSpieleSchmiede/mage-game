---
title: "P-005 — Avians Physik in FixedUpdate einhängen und ordnen (erledigt)"
---
# P-005 — Avians Physik in FixedUpdate einhängen und ordnen (erledigt)

Erledigt mit B-005 (2026-11-15). Die Befürchtung hat sich bestätigt: Mit Avians Standard-Schedule `FixedPostUpdate` sah ein Shape-Cast im Set Treffer die Position eines Magiers, der sich im selben Tick bewegt hatte, nicht. Lösung in `MageSimPlugin`: `PhysicsPlugins::new(FixedUpdate)` und Avians Sets `PhysicsSystems::First` bis `Last` sowie `SpatialQuerySystems` im Set `SimSet::Physics`. Festgeschrieben in `crates/mage_sim/tests/tick_chain.rs`.
