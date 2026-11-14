---
title: "P-012 — Fluent-Integration in Bevy"
---
# P-012 — Fluent-Integration in Bevy

Texte liegen als Fluent-Dateien vor ([[adr-0023-texte-ueber-fluent-schluessel|ADR-0023]]). Offen ist, ob eine fertige Bevy-Crate genutzt wird (muss Bevy 0.20 unterstützen, Dateien über den Asset-Server laden und damit auch im Browser und auf Mobilgeräten funktionieren) oder eine schlanke eigene Schicht über die `fluent`-Crate. Außerdem: Sprachwahl beim ersten Start nach Systemsprache, Wechsel zur Laufzeit.

Nächster Schritt: in M3 vor dem ersten HUD-Text entscheiden und den Vollständigkeitstest ([[lokalisierung|Lokalisierung]]) als erstes schreiben.
