---
title: "Glossar"
---
# Glossar

| Begriff | Bedeutung |
| --- | --- |
| MageIntent | serialisierbare Per-Tick-Steuerungsdaten eines Magiers (Bewegung, Zielrichtung, Knopfzustände) |
| InputBuffer | Ressource, in der lokale Intent-Produzenten zwischen den Ticks Eingaben sammeln; das Set Intent friert sie zu einem MageIntent ein |
| Tick | ein fixer Simulationsschritt; alle Dauern sind Tick-Zähler |
| Latch | Zähler für Druckereignisse zwischen zwei Ticks im InputBuffer; die Simulation konsumiert und resettet ihn |
| Flanke | Übergang gerade gedrückt/gerade losgelassen, abgeleitet aus Latch und Vor-Tick-Zustand |
| Blueprint | serialisierbare Zauber-Konfiguration in der Registry, Schlüssel `SpellId` |
| Delivery | wie ein Zauber zum Ziel kommt: `Projectile` (Flugbahn) oder `GroundArea` (Bodenpunkt mit Fuse) |
| Modifikator | Component an der Zauber-Entity mit einem eigenen System und genau einem Lifecycle-Haken |
| Effekt | reine Daten im on-Impact-Pfad: `Damage`, `AreaDamage`, `Slow` |
| LOS | Line of Sight; Sichtlinie, die Arena-Kollider blockieren (Verbraucher: Homing) |
| Fuse | Tick-Zähler eines GroundArea-Zaubers bis zum Impact |
| PlayerId / SpellId | stabile IDs für Magier und Blueprints; ersetzen Entities in jedem serialisierten Datum |
| SimId | fortlaufende ID jeder Simulations-Entity; bestimmt jede ergebnisrelevante Reihenfolge und liegt im Snapshot |
| Headless | Simulations-Plugins ohne Fenster und Renderer, Normalfall der Tests |
| Greybox | untexturierte Platzhaltergeometrie der Arena |
| Rollback | Netcode-Ansatz: bei verspäteter oder abweichender Eingabe zum betroffenen Tick zurückspulen (Snapshot) und neu rechnen; bei uns mit dem Server als Schiedsrichter |
| ADR | Architecture Decision Record; nummerierte Entscheidung mit verworfener Alternative |
| P-Log | Problem-Log; nummerierte offene Fragen mit nächstem Schritt |
| Team / TeamId | Gruppe von Magiern, die zusammen spielen; „Gegner“ heißt immer: anderes Team |
| Szenario | Beschreibung einer Runde: Teams, Spieler, Arena, Seed; gemeinsam für Tests, Benchmarks, KI und Lobby |
| Attribut | Eigenschaft eines Magiers (Tempo, Zauberkraft, Tarnung …); Basiswert plus Modifikatoren ergibt den effektiven Wert |
| Attribut-Modifikator | zeitlich begrenzte additive oder multiplikative Änderung eines Attributs, z. B. Slow |
| VisibleTo | Bitmaske je Entity: welche Teams sie gerade sehen |
| Tarnung / Wahrnehmung | Attribute, die über Sichtbarkeit entscheiden; Unsichtbarkeit erhöht die Tarnung |
| Enthüllung | kurze Zeit, in der ein Magier unabhängig von Tarnung sichtbar ist (nach Zaubern oder Schaden) |
| Beobachtung (Observation) | alles, was ein Spieler in einem Tick wissen darf; einzige Leseschnittstelle für HUD, KI, Add-ons, Netzwerk |
| Aktion | geräteunabhängige Eingabe (Bewegen, Zauber 1 …), auf Tasten, Sticks oder Touch belegbar |
| Fluent | Format für übersetzte Texte (`.ftl`), kann Mehrzahl und Variablen |
| Server als Schiedsrichter | der Server führt die maßgebliche Simulation; weicht ein Client ab, gilt der Server |
| Vorhersage (fremder Eingaben) | fehlt der Intent eines Mitspielers noch, gilt dessen letzter Intent weiter, bis der echte ankommt |
| Korrektur (Abgleich) | Client spult bei Abweichung zum Server-Stand zurück, übernimmt ihn und rechnet seine Eingaben neu |
| Interest Management | Server schickt jedem Client nur, was dessen Team sehen darf |
| Add-on | Spieler-Erweiterung in einer Sandbox: liest die Beobachtung, schlägt Intents vor |
| RL (Reinforcement Learning) | Lernverfahren, bei dem Agenten durch Belohnungen aus vielen Spielen lernen |
| Backlog-Item (B-nnn) | konkrete nächste Aufgabe mit Abhängigkeiten und Tests, im Backlog |
| Zelle / CellId | Quadrat des Weltrasters; Einheit, in der die Welt geladen und entladen wird |
| Interessenbereich | Zellen im Radius R um den eigenen Magier; nur dort existiert für einen Client etwas Simulationsrelevantes |
| Fernsicht | grobe Landschaft außerhalb des Interessenbereichs, reine Präsentation ohne Gameplay |
| Einheit | alles, was kämpfen kann: Magier und Kreaturen, mit gemeinsamem Kern (Team, Leben, Attribute, Leiste, Steuerung) |
| Kreatur | Einheit aus einem `CreatureBlueprint`, meist KI-gesteuert: Tier, Gegner, Pet, Reittier, Boss |
| Reittier (Mount) | Kreatur mit eigenem Leben, auf der ein Magier reitet oder fliegt |
| Kampfzustand | außer Kampf, im Kampf oder tot; entscheidet, ob Leiste und Ausrüstung geändert werden dürfen |
| Fähigkeitenleiste (Loadout) | vom Spieler deklarierte Zuordnung Slot → Fähigkeit; Teil der Simulation |
