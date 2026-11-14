---
title: "P-016 — Direkte Verbindungen zwischen Clients als Optimierung"
---
# P-016 — Direkte Verbindungen zwischen Clients als Optimierung

Alle Eingaben laufen über den Server ([[adr-0017-rollback-mit-server-als-schiedsrichter|ADR-0017]]). Direkte Verbindungen zwischen Clients würden fremde Eingaben früher zustellen (typisch 10 bis 40 ms) und damit seltener und flacher zurückspulen lassen. Dagegen stehen:

- **Der Server braucht trotzdem alle Eingaben.** Er rechnet mit, um Widersprüche zu entscheiden; gespart wird nur die Weiterleitung, nicht der Verkehr zum Server.
- **Unsichtbarkeit:** Nur der Server weiß, wer wen sieht. Ein Client kann nicht sicher entscheiden, wann er einem Gegner keine Eingaben mehr schicken darf, und ein manipulierter Client würde es absichtlich weiter tun ([[sicherheit|Sicherheit]]).
- **Abweichende Eingaben:** Ein Client kann Mitspielern andere Eingaben schicken als dem Server; der Server gewinnt, aber das erzeugt genau die Korrekturen, die man vermeiden wollte.
- **IP-Adressen** werden für Mitspieler sichtbar.
- **Technik:** NAT-Durchdringung mit Relay-Rückfall, im Browser nur über WebRTC, auf Mobilgeräten Upload an viele Gegenstellen ([[plattformen|Plattformen]]).

**Bedingungen, falls es kommt:**

1. Nur **zusätzlich** zum Weg über den Server, nie statt ihm; der Server bleibt Schiedsrichter.
2. Zuerst nur **innerhalb eines Teams**: Verbündete sehen sich immer, das Unsichtbarkeitsproblem entfällt, der Anreiz zum Betrug ist gering.
3. **Opt-in** pro Spieler, wegen der IP-Adressen.
4. Zwischen Gegnern höchstens in Spielmodi ohne Unsichtbarkeit, und nur wenn die Messungen es rechtfertigen.

**Schon jetzt in der Architektur:** Weitergeleitete Intents tragen Absender (`PlayerId`), Tick und Sequenznummer und sind unabhängig vom Transportweg; ein Client nimmt für jeden Tick und Spieler den ersten eintreffenden Intent und verwirft Doppelte. Damit kostet ein direkter Weg später keinen Umbau.

Nächster Schritt: in M7 mit den Latenztests ([[synchronisation-latenz|Synchronisation & Latenz]]) messen, wie oft und wie tief über den Server zurückgespult wird. Lohnt es sich, als ADR entscheiden: zuerst direkte Verbindungen innerhalb eines Teams, als Opt-in.
