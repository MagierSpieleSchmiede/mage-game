---
title: "Synchronisation & Latenz — Netzwerk testen ohne Netzwerk"
---
# Synchronisation & Latenz — Netzwerk testen ohne Netzwerk

Netzwerkverhalten wird nicht erst im echten Netz getestet. Ein Test startet Server und mehrere Clients **in einem Prozess** und verbindet sie über eine **simulierte Leitung**, die Latenz, Schwankung (Jitter), Paketverlust, Vertauschung und Bandbreite einstellbar macht. Die Zeit läuft in Ticks, nicht nach der Uhr; dadurch sind die Tests deterministisch und schnell. Gebaut in Milestone 7 ([[netzwerk|Netzwerk]]); die Tests werden zuerst geschrieben ([[adr-0026-tests-zuerst|ADR-0026]]).

## Leitungsprofile

| Profil | Latenz (eine Richtung) | Jitter | Verlust |
| --- | --- | --- | --- |
| LAN | 1 ms | 0 | 0 % |
| gut | 25 ms | 5 ms | 0,5 % |
| mittel | 50 ms | 15 ms | 2 % |
| schlecht | 100 ms | 40 ms | 5 % |
| Mobilfunk | 150 ms | 80 ms | 10 % |

## Tests

| Test | Erwartung |
| --- | --- |
| **Konvergenz** | Nachdem alle aufhören zu steuern, ist der Zustand jedes Clients (für alles, was er sieht) nach spätestens Puffer + Latenz + 2 Ticks gleich dem Server-Zustand. |
| **Vorhersage ohne Verlust** | Im Profil LAN und gut korrigiert der Server die Vorhersage des eigenen Magiers nie (Prüfsummen gleich). |
| **Abgleich** | Bei Verlust weicht die Vorhersage ab und wird korrigiert; die Korrektur ist nach einem Server-Stand vollständig, und alle späteren Eingaben bleiben erhalten. |
| **Kein Eingabeverlust** | Jeder Tastendruck eines Clients wirkt auf dem Server genau einmal, auch bei 10 % Verlust (dank Wiederholung der letzten Intents). |
| **Eingabeverzögerung** | Ein Intent wirkt auf dem Server spätestens Latenz + Jitter-Puffer + 1 Tick nach dem Senden; der Client sieht die Wirkung beim eigenen Magier sofort (Vorhersage). |
| **Sichtbarkeitsfilter** | Kein Paket an Team A enthält eine Entity, die für Team A unsichtbar ist; geprüft an jedem gesendeten Paket. |
| **Fremde Magier flüssig** | Interpolierte Positionen springen bei Profil mittel nicht weiter als eine Platzhalter-Grenze pro Bild. |
| **Treffer unter Latenz** | Anteil der Treffer, die auf dem Client sichtbar trafen, auf dem Server aber nicht, je Profil; Grundlage für [[p-015-trefferwertung-unter-latenz\|P-015]]. |
| **Uhrabgleich** | Nach einem plötzlichen Latenzsprung von 25 auf 100 ms passt der Client seinen Vorlauf innerhalb einer Platzhalter-Zahl von Ticks an. |
| **Bandbreite** | Bytes pro Sekunde je Client im 5-gegen-5-Szenario unter einer Grenze ([[benchmarks\|Benchmarks]]). |
| **Wiederverbinden** | Ein Client, der 2 s nichts empfängt, bekommt danach einen vollen Zustand und ist wieder synchron. |

## Messgrößen für Berichte

Korrekturen pro Minute, mittlere und maximale Korrekturweite, Eingabeverzögerung (Mittel, 95. Perzentil), Bandbreite pro Client, Prüfsummen-Abweichungen. Werden bei jedem Lauf in die Testausgabe geschrieben, damit Veränderungen auffallen.
