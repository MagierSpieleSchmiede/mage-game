---
title: "Erweiterungspunkte — wo das System wachsen soll, ohne umgebaut zu werden"
---
# Erweiterungspunkte — wo das System wachsen soll, ohne umgebaut zu werden

| Erweiterung | Schnittstelle | Seite |
| --- | --- | --- |
| Mehr Teams und Spieler | `TeamId`, Szenario-Format | [[teams\|Teams]] |
| Neue Eigenschaften und Effekte | Attribut-Enum, Modifikatoren als Daten | [[attribute\|Attribute]] |
| Unsichtbarkeit, Wahrnehmung, Enthüllung | `VisibleTo`, Tarnung und Wahrnehmung als Attribute | [[sichtbarkeit\|Sichtbarkeit]] |
| KI-Gegner und RL-Training | Beobachtung lesen, `MageIntent` schreiben | [[ki-und-rl\|KI & RL]] |
| Add-ons (auslesen, automatisieren) | dieselbe Schnittstelle, in einer Sandbox | [[add-ons\|Add-ons]] |
| Netzwerk-PvP | Rollback auf den Clients, Server als Schiedsrichter; Intents und Zustand nur für Sichtbares | [[netzwerk\|Netzwerk]] |
| Eingabegeräte | Aktionen und Belegungen | [[eingabegeraete\|Eingabegeräte]] |
| Sprachen | Fluent-Ordner pro Sprache | [[lokalisierung\|Lokalisierung]] |
| Plattformen | Stufen-Regeln | [[plattformen\|Plattformen]] |

**Snapshot und Restore** (für Rollback auf den Clients, Server-Korrekturen und Replays): Zu snapshotten sind alle Simulations-Components (`SimId`, `TeamId`, Position und Richtung von Magiern und Zaubern, Health, Mana, Cooldowns, Basiswerte und aktive Attribut-Modifikatoren, `VisibleTo` und Enthüllungen, `IntentForTick`, Zustände laufender Zauber inklusive Fuse und Lebensdauer, `Dead` und Respawn-Timer) und die Ressourcen mit Zustand (RNG, `SimId`-Zähler; Balance ist pro Runde konstant und bleibt außen vor). Dazu kommt eine Intent-Historie pro Tick, aus der das Set Intent bei der Neuberechnung liest. Explizit nicht gesnapshottet: statische Arena-Kollider und Avian-Interna, weil die Simulation Avian nur abfragt ([[adr-0003-kinematik-statt-dynamischer-rigidbodies|ADR-0003]], [[adr-0007-shape-cast-pro-tick-fuer-projektile|ADR-0007]]). Restore heißt: Simulations-Entities entfernen, Zustand aus dem Snapshot wiederherstellen, Bindung `PlayerId` → Entity neu aufbauen, Avians Query-Struktur einmal auffrischen, Ticks ab dort neu ausführen. Weil jede Reihenfolge an der `SimId` hängt, dürfen die Entities dabei neue Indizes bekommen ([[adr-0013-simid-statt-entity-index-fuer-jede|ADR-0013]]).

**Neues Delivery:** Enum-Variante in `Delivery`, ein System im Set Bewegung, eins im Set Treffer, eine Blueprint-Zeile, fertig. Beispiel-Delivery später: `Beam` (Soforttreffer entlang aim_dir per Raycast).

**Neuer Modifikator:** Component plus genau ein Lifecycle-System (on cast, per tick, on impact, on expire) und eine Enum-Variante in der Registry. Bestehende Zauber unberührt.

**Neuer Effekt:** Enum-Variante plus ein Handler im einzigen Impact-Pfad (`SpellImpact` → Effekte → `DamageApplied`). Zeitlich begrenzte Effekte sind Attribut-Modifikatoren. Es gibt keinen zweiten Schadenspfad, in dem man ihn vergessen könnte.
