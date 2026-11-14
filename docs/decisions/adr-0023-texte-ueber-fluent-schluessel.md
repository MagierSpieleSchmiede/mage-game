---
title: "ADR-0023 — Alle Texte über Fluent-Schlüssel; Englisch und Deutsch von Anfang an"
---
# ADR-0023 — Alle Texte über Fluent-Schlüssel; Englisch und Deutsch von Anfang an

**Status:** vorgeschlagen

**Kontext:** Das Spiel soll mindestens auf Englisch und Deutsch laufen. Texte nachträglich aus dem Code zu holen ist mühsam und fehleranfällig.

**Entscheidung:** Kein sichtbarer Text steht im Code. Jeder Text ist ein Schlüssel in Dateien im Format [Fluent](https://projectfluent.org) (`.ftl`), je Sprache ein Ordner (`en`, `de`). Englisch ist die Rückfallsprache. Die Simulation enthält gar keine Texte; Zauber, Attribute und Effekte haben stabile IDs, und die Präsentation bildet sie auf Schlüssel ab. Ein Test prüft, dass jeder Schlüssel in jeder Sprache existiert und jeder verwendete Schlüssel definiert ist.

**Verworfene Alternative:** Texte zunächst fest auf Englisch, Übersetzung später. Verworfen, weil das Herausziehen später jede UI-Stelle anfasst und Grammatik (Mehrzahl, Fälle) dann nachträglich passen muss, was Fluent von Anfang an abdeckt.

**Konsequenzen:** Etwas Aufwand bei jedem neuen Text. Welche Bevy-Integration für Fluent, ist offen ([[p-012-fluent-integration|P-012]]). Details: [[lokalisierung|Lokalisierung]].
