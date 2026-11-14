---
title: "Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt"
---
# Status — Übergabe: was zuletzt erledigt wurde und was als Nächstes kommt

> [!important] Wozu diese Seite da ist
> Sie ist die **Übergabe von einer Sitzung zur nächsten**, damit niemand den ganzen Vault lesen muss. Wer eine Aufgabe beginnt, liest **nur diese Seite** und die Seiten, die unter *Dafür lesen* stehen. Wer eine Aufgabe beendet, **überschreibt** die drei Abschnitte unten (nicht anhängen; der Verlauf gehört ins Implementation-Log des Milestones). Kurz halten: höchstens etwa 30 Zeilen.

## Zuletzt erledigt

**2026-11-15, Planung und B-004 (Branch `dev-danielb`):**

- **Neue Vorgaben geplant** (alle *vorgeschlagen*): große Welt mit Interessenbereich ([[adr-0028-grosse-welt-mit-interessenbereich|ADR-0028]], [[welt|Welt]]), Reittiere am Boden und in der Luft ([[adr-0029-reittiere-als-bewegungsmodus|ADR-0029]], [[reittiere|Reittiere]]), vom Spieler deklarierte Fähigkeitenleiste und Belegung ([[adr-0030-faehigkeitenleiste-vom-spieler|ADR-0030]], [[faehigkeiten|Fähigkeiten & Belegung]]). Offene Fragen P-017 bis P-021, Milestones M10 und M11, Backlog-Item B-028.
- **B-004:** `mage_sim::ids` (`SimId` u64 mit `SimIdCounter`, `PlayerId` u32, `TeamId` u8, `SpellId` u16) und `mage_sim::mage` (Marker `Mage`, `spawn_scenario_mages` beim Start). Tests in `crates/mage_sim/tests/ids.rs`.

## Nächste Aufgabe

**B-005 — Tick-Kette als `SystemSet`s (Intent, Werte, Casting, Bewegung, Physik, Treffer, Schaden, Sichtbarkeit, Aufräumen), Avian in `FixedUpdate` als Set Physik.**

- **Erst klären:** nichts; das Set Sichtbarkeit bleibt leer (ADR-0018 ist noch vorgeschlagen). Ein Set *Welt* vor Intent kommt erst, wenn ADR-0028 angenommen ist.
- **Dafür lesen:** die Zeile B-005 im [[backlog|Backlog]]; [[simulationsmodell|Simulationsmodell]] (Kette und Tabelle der Sets); [[p-005-avians-physik-in-fixedupdate-einhaengen|P-005]]. Avian 0.8 im Quellcode prüfen: wie `PhysicsPlugins` einen Schedule bekommt und welche Sets es gibt (`~/.cargo/registry/src/*/avian3d-0.8.0/`).
- **Tests zuerst:** Reihenfolge der Sets; ein Shape-Cast im Set Treffer sieht die Magier-Position desselben Ticks (dafür braucht der Test einen Kollider am Magier; eine Bewegung von Hand im Set Bewegung genügt).

## Hinweise für die nächste Sitzung

- Magier werden in `Startup` gespawnt; `headless_app` führt `Startup` schon aus, bevor es die App zurückgibt.
- Nach jeder Aufgabe ohne Nachfrage `dev-danielb` pushen, CI abwarten, bei Grün `master` per Fast-Forward nachziehen (`AGENTS.md`).
- `Resource` ist in Bevy 0.20 ein Subtrait von `Component`: Ein Typ kann nicht beides ableiten.
