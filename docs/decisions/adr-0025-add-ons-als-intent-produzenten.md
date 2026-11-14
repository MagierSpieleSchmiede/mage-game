---
title: "ADR-0025 — Add-ons lesen nur die Beobachtung und schreiben nur Intents, in einer Sandbox"
---
# ADR-0025 — Add-ons lesen nur die Beobachtung und schreiben nur Intents, in einer Sandbox

**Status:** vorgeschlagen

**Kontext:** Spieler sollen Add-ons nutzen können, die den eigenen Charakter auslesen (Anzeigen, Statistiken) oder Aktionen automatisieren. Add-ons sind fremder Code auf dem Rechner des Spielers und können im PvP ein Vorteil sein.

**Entscheidung:** Ein Add-on ist ein Modul in einer **Sandbox** (WebAssembly) mit genau zwei Fähigkeiten: Es bekommt pro Tick die Beobachtung seines Spielers ([[adr-0019-eine-beobachtungs-schnittstelle|ADR-0019]]) und darf einen `MageIntent` vorschlagen. Kein Dateisystem, kein Netzwerk, keine Uhr außer dem Tick, begrenzte Rechenzeit und begrenzter Speicher pro Tick. Der Intent eines Add-ons geht denselben Weg wie menschliche Eingabe und wird vom Server genauso geprüft. Jede Spielrunde legt fest, was erlaubt ist: keine Add-ons, nur Auslesen, oder auch Automatisieren.

**Verworfene Alternative:** Skript-Zugriff auf die ECS-Welt oder native Plugins (DLLs). Verworfen, weil sie versteckte Information lesen und beliebigen Code auf dem Rechner ausführen könnten.

**Konsequenzen:** Add-ons sehen nie mehr als der Spieler selbst. Automatisierung kann der Server nicht sicher von menschlicher Eingabe unterscheiden; das bleibt eine Regel- und Vertrauensfrage pro Spielmodus ([[p-010-add-on-laufzeit-und-fairness|P-010]]). Details: [[add-ons|Add-ons]].
