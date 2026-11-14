---
title: "ADR-0016 — Attribute mit Modifikatoren, pro Tick zu effektiven Werten verrechnet"
---
# ADR-0016 — Attribute mit Modifikatoren, pro Tick zu effektiven Werten verrechnet

**Status:** angenommen (2026-11-15)

**Kontext:** Ein Spieler soll mehrere Eigenschaften haben, die sagen, wie stark und fähig er gerade ist (Tempo, Zauberkraft, Widerstand, Tarnung …), und die sich durch Effekte zeitweise ändern. Bisher war `Slow` ein Sonderfall mit eigener Abklinglogik.

**Entscheidung:** Jeder Magier hat **Basiswerte** (`BaseAttributes`) und eine Liste **aktiver Modifikatoren** (Attribut, Art: additiv oder multiplikativ, Wert, Restdauer in Ticks, Quelle als `SimId`). Ein eigenes Set **Werte** direkt nach Intent zählt Restdauern herunter, entfernt abgelaufene Modifikatoren und berechnet `EffectiveAttributes`: (Basis + Summe der additiven) × Produkt der multiplikativen, geklemmt auf Grenzen. Gerechnet wird ganzzahlig (Tausendstel), damit es deterministisch bleibt. Alle anderen Systeme lesen nur `EffectiveAttributes`. `Slow` wird ein gewöhnlicher Modifikator auf Bewegungstempo.

**Verworfene Alternative:** Jede Eigenschaft als eigene Component mit eigener Logik (wie bisher bei Slow). Verworfen, weil Stapelregeln, Dauer und Anzeige dann für jede Eigenschaft neu und uneinheitlich gebaut würden.

**Konsequenzen:** Neue Effekte (Schild, Eile, Schwäche, Tarnung) sind Daten statt Code. HUD, KI und Add-ons lesen dieselben Werte über die Beobachtung ([[adr-0019-eine-beobachtungs-schnittstelle|ADR-0019]]). Die Set-Kette bekommt ein Set mehr. Details: [[attribute|Attribute]].
