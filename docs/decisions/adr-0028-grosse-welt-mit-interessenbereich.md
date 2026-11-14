---
title: "ADR-0028 — Große Welt in Zellen; geladen wird nur der Interessenbereich um den Spieler"
---
# ADR-0028 — Große Welt in Zellen; geladen wird nur der Interessenbereich um den Spieler

**Status:** vorgeschlagen

**Kontext:** Das Spiel soll neben der Arena eine große Welt haben, mit potenziell tausenden Spielern gleichzeitig. Kein Client kann die ganze Welt im Speicher halten oder simulieren, und kein Client darf erfahren, was er nicht sehen kann ([[adr-0018-sichtbarkeit-als-simulationszustand|ADR-0018]]). Bisher ging die Planung von einer kleinen Arena aus, die immer vollständig geladen ist.

**Entscheidung:**

1. Die Welt ist in **Zellen** eines festen Rasters geteilt (Platzhalter 64 m × 64 m). Eine Zelle enthält statische Kollider, Spawnpunkte und Weltobjekte; sie ist Daten, die geladen und entladen werden kann.
2. Jeder Spieler hat einen **Interessenbereich**: alle Zellen im Radius R um seinen Magier (Platzhalter 160 m). Nur dort existieren für ihn simulationsrelevante Dinge: Kollider, Magier, Zauber, Weltobjekte. Was außerhalb liegt, ist für seinen Client nicht im Speicher.
3. **Laden und Entladen mit Hysterese:** Zellen werden ab R geladen und erst ab R + Δ (Platzhalter 32 m) entladen, damit Bewegung an der Grenze nicht ständig lädt. Laden und Entladen geschieht an Tick-Grenzen; wann, entscheidet die Simulation, nicht die Bildrate.
4. **Fernsicht ist reine Präsentation:** Grobe Landschaft außerhalb von R (Gelände in niedriger Auflösung, Silhouetten großer Bauwerke) darf geladen und gezeigt werden, enthält aber keinerlei Gameplay-Information: keine Magier, keine Zauber, keine Kollider.
5. **Der Server** kennt die ganze Welt, leitet jedem Client aber nur Intents und Zustand aus dessen Interessenbereich weiter, geschnitten mit der Sichtbarkeit (Interest Management nach Entfernung und nach Sichtbarkeit, [[adr-0017-rollback-mit-server-als-schiedsrichter|ADR-0017]]).
6. **Die Arena bleibt** der Testfall und ein Spielmodus: Sie ist eine Welt aus wenigen Zellen, die immer vollständig im Interessenbereich liegt. Alle M1-Tests laufen dort.

**Verworfene Alternativen:** Die ganze Welt auf jedem Client: verworfen wegen Speicher, Rechenzeit und weil Information über Unsichtbares im Client läge. Instanzierte kleine Gebiete mit Ladebildschirmen: verworfen, weil eine zusammenhängende Welt gewünscht ist; Instanzen bleiben als Spielmodus möglich.

**Konsequenzen:** Simulations-Entities kommen und gehen nicht nur durch Spawn und Tod, sondern auch durch Laden und Entladen; ihre `SimId` muss deshalb weltweit eindeutig sein und vom Server vergeben werden ([[serialisierung-ids|Serialisierung & IDs]]). Rollback auf dem Client gilt nur innerhalb des Interessenbereichs. Ein einzelner Serverprozess wird nicht tausende Spieler bei 60 Ticks rechnen können; wie die Welt auf Server verteilt wird, ist offen ([[p-017-welt-auf-mehrere-server-verteilen|P-017]]). Details: [[welt|Welt]].
