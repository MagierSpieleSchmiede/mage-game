---
title: "Entscheidungen — alle ADRs mit ihrem Stand"
---
# Entscheidungen — alle ADRs mit ihrem Stand

Die eine Liste aller Architekturentscheidungen. Der **Stand** steht nur hier und in der ADR selbst; andere Seiten verlinken die ADR, ohne ihren Stand zu wiederholen. Neue ADRs nach der [[adr-nnnn|ADR-Vorlage]] und den [[konventionen|Konventionen]]. Nächste freie Nummer: **ADR-0033**.

*Vorgeschlagen* heißt: Planungsgrundlage, aber vor Code, der darauf baut, Daniels Freigabe einholen.

| Nr. | Entscheidung | Stand |
| --- | --- | --- |
| [[adr-0001-tick-simulation-in-fixedupdate-statt-delta\|ADR-0001]] | Tick-Simulation in FixedUpdate statt Delta-Zeit in Update | angenommen |
| [[adr-0002-intent-als-serialisierbare-daten-statt-eingabe\|ADR-0002]] | Intent als serialisierbare Daten statt Eingabe-Lesen in der Simulation | angenommen |
| [[adr-0003-kinematik-statt-dynamischer-rigidbodies\|ADR-0003]] | Kinematik statt dynamischer RigidBodies: Avian nur für statische Kollider und Queries | angenommen |
| [[adr-0004-latch-zaehler-fuer-flanken\|ADR-0004]] | Latch-Zähler für Flanken statt Vor-Tick-Ableitung allein | angenommen |
| [[adr-0005-observer-sind-der-eine-codepfad\|ADR-0005]] | Observer sind der eine Codepfad für Schaden und Tod; Mana gehört dem Casting-System | angenommen |
| [[adr-0006-serde-statt-bevy-reflect-fuer-intent\|ADR-0006]] | serde statt bevy_reflect für Intent und Blueprints | angenommen |
| [[adr-0007-shape-cast-pro-tick-fuer-projektile\|ADR-0007]] | Shape-Cast pro Tick für Projektile statt Kollisions-Events | angenommen |
| [[adr-0008-ticks-statt-sekunden-fuer-alle-timer\|ADR-0008]] | Ticks statt Sekunden für alle Timer | angenommen |
| [[adr-0009-eine-geseedete-rng-ressource-statt-verstreuter\|ADR-0009]] | Eine geseedete RNG-Ressource statt verstreuter Zufallsquellen | angenommen |
| [[adr-0010-spread-modifikator-als-der-rng-verbraucher\|ADR-0010]] | Spread-Modifikator als der RNG-Verbraucher des Prototyps | angenommen |
| [[adr-0011-low-res-render-target-mit-nearest\|ADR-0011]] | Low-Res-Render-Target mit Nearest-Upscale | angenommen |
| [[adr-0012-split-hakt-auf-on-impact\|ADR-0012]] | Split hakt auf on impact statt on expire | angenommen |
| [[adr-0013-simid-statt-entity-index-fuer-jede\|ADR-0013]] | SimId statt Entity-Index für jede ergebnisrelevante Reihenfolge | angenommen |
| [[adr-0014-impacts-sammeln-dann-sortiert-ausloesen\|ADR-0014]] | Impacts sammeln, dann sortiert auslösen | angenommen |
| [[adr-0015-teams-statt-fester-zwei-spieler\|ADR-0015]] | Teams mit beliebig vielen Magiern statt eines festen Zwei-Spieler-Duells | angenommen (2026-11-14) |
| [[adr-0016-attribute-mit-modifikatoren\|ADR-0016]] | Attribute mit Modifikatoren, pro Tick zu effektiven Werten verrechnet | **vorgeschlagen** |
| [[adr-0017-rollback-mit-server-als-schiedsrichter\|ADR-0017]] | Rollback-Netcode, wo immer möglich; der Server ist Schiedsrichter | angenommen (2026-11-14) |
| [[adr-0018-sichtbarkeit-als-simulationszustand\|ADR-0018]] | Sichtbarkeit ist Simulationszustand, pro Team berechnet | **vorgeschlagen** |
| [[adr-0019-eine-beobachtungs-schnittstelle\|ADR-0019]] | Eine Beobachtungs-Schnittstelle für HUD, KI, Reinforcement Learning und Add-ons | **vorgeschlagen** |
| [[adr-0020-simulation-als-eigene-crate\|ADR-0020]] | Cargo-Workspace mit der Simulation als eigener Crate ohne Render-Abhängigkeiten | angenommen (2026-11-14) |
| [[adr-0021-animation-ist-reine-praesentation\|ADR-0021]] | Animation ist reine Präsentation; Fähigkeiten laufen nach Ticks | **vorgeschlagen** |
| [[adr-0022-eingabe-ueber-aktionen\|ADR-0022]] | Eingabe über Aktionen statt über Tasten (Tastatur und Maus, Controller, Touch) | **vorgeschlagen** |
| [[adr-0023-texte-ueber-fluent-schluessel\|ADR-0023]] | Alle Texte über Fluent-Schlüssel; Englisch und Deutsch von Anfang an | **vorgeschlagen** |
| [[adr-0024-plattformneutralitaet\|ADR-0024]] | Plattformneutralität: keine Entscheidung, die eine Zielplattform ausschließt | angenommen (2026-11-14) |
| [[adr-0025-add-ons-als-intent-produzenten\|ADR-0025]] | Add-ons lesen nur die Beobachtung und schreiben nur Intents, in einer Sandbox | **vorgeschlagen** |
| [[adr-0026-tests-zuerst\|ADR-0026]] | Tests zuerst: jede Funktion beginnt mit einem fehlschlagenden Test | angenommen (2026-11-14) |
| [[adr-0027-zielfilter-pro-faehigkeit\|ADR-0027]] | Friendly Fire wird pro Fähigkeit über einen Zielfilter entschieden | angenommen |
| [[adr-0028-grosse-welt-mit-interessenbereich\|ADR-0028]] | Große Welt in Zellen; geladen wird nur der Interessenbereich um den Spieler | **vorgeschlagen** |
| [[adr-0029-reittiere-als-bewegungsmodus\|ADR-0029]] | Reittiere sind Kreaturen mit eigenem Leben, am Boden oder fliegend; der Reiter steuert und kämpft im Sattel | **vorgeschlagen** (überarbeitet) |
| [[adr-0030-faehigkeitenleiste-vom-spieler\|ADR-0030]] | Der Spieler stellt seine Fähigkeitenleiste und ihre Tastenbelegung selbst zusammen | angenommen (2026-11-15) |
| [[adr-0031-einheiten-magier-und-kreaturen\|ADR-0031]] | Magier und Kreaturen sind Einheiten mit gemeinsamem Kern; Bosse, Pets und Tiere sind Kreaturen | angenommen (2026-11-15) |
| [[adr-0032-kampfzustand\|ADR-0032]] | Jede Einheit ist außer Kampf, im Kampf oder tot; davon hängt ab, was am Charakter geändert werden darf | **vorgeschlagen** |

## Was offen ist

Zehn vorgeschlagene ADRs warten auf Freigabe. Als Nächstes braucht Milestone 1 ADR-0016 (Attribute, B-007); ADR-0032 (Kampfzustand) wartet auf die Frage, ob man im Tod den Charakter ändern darf.
