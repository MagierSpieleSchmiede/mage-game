---
title: "ADR-0005 — Observer sind der eine Codepfad für Schaden und Tod; Mana gehört dem Casting-System"
---
# ADR-0005 — Observer sind der eine Codepfad für Schaden und Tod; Mana gehört dem Casting-System

**Kontext:** Invariante: Schaden, Mana-Verbrauch und Tod je genau ein Pfad; gleichzeitig sollen diskrete Events Observer sein.
**Entscheidung:** Für Schaden und Tod ist der Observer-Handler der eine Pfad. HP wird ausschließlich vom `DamageApplied`-Handler verändert, Mana nur vom Casting-System, Tod nur vom `MageDied`-Handler.
**Verworfene Alternative:** Zentrale `apply_damage`-Funktion plus zusätzlich Observer für UI. Verworfen, weil zwei Pfade entstehen, die auseinanderlaufen.
**Konsequenzen:** Jede neue Schadensquelle ist nur ein neuer Auslöser, kein neuer Effekt-Code.
