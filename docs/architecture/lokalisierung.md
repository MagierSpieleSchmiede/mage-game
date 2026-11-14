---
title: "Lokalisierung — Englisch und Deutsch, alle Texte über Schlüssel"
---
# Lokalisierung — Englisch und Deutsch, alle Texte über Schlüssel

Das Spiel läuft mindestens auf Englisch und Deutsch. Weitere Sprachen sind ein Ordner mehr. Entschieden in [[adr-0023-texte-ueber-fluent-schluessel|ADR-0023]].

## Regeln

- **Kein sichtbarer Text im Code.** Jeder Text ist ein Schlüssel, zum Beispiel `spell-bolt-name` oder `hud-mana`.
- **Fluent-Dateien** (`.ftl`) unter `assets/locales/<sprache>/`, je Bereich eine Datei (`hud.ftl`, `spells.ftl`, `menu.ftl`). Fluent kann Mehrzahl und Variablen (`{ $count ->` …), das braucht Deutsch.
- **Englisch ist Rückfall:** fehlt ein Schlüssel in einer Sprache, wird Englisch gezeigt und im Debug-Build gewarnt.
- **Die Simulation kennt keine Texte.** Zauber, Attribute, Effekte und Ereignisse haben IDs; die Präsentation bildet sie auf Schlüssel ab (`SpellId` → `spell-<name>-name`).
- **Sprache:** beim ersten Start die Systemsprache, sonst Englisch; im Spiel umstellbar, ohne Neustart.
- **Schriftart** mit allen Zeichen der unterstützten Sprachen (Umlaute, ß; später ggf. weitere Schriften).

## Tests

- Jeder Schlüssel existiert in jeder Sprache, und jede Sprachdatei parst fehlerfrei.
- Jeder im Code verwendete Schlüssel ist definiert (Schlüssel als Konstanten, Test listet sie).
- Fenster-Test mit Deutsch, weil deutsche Texte oft länger sind: nichts läuft aus dem HUD.

## Was offen ist

Welche Bevy-Integration: [[p-012-fluent-integration|P-012]].
