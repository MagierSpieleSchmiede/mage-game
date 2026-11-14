---
title: "Sicherheit — wie Server und PvP-Kommunikation abgesichert werden"
---
# Sicherheit — wie Server und PvP-Kommunikation abgesichert werden

> [!info] Stand
> Geplant; es gibt noch keinen Server und keinen Netzwerkcode. Diese Seite legt fest, wie die Sicherheit **umgesetzt werden soll**, und wird beim Bauen zur Beschreibung dessen, was **umgesetzt ist**. Jede Abweichung wird hier nachgetragen. Wie man eine Sicherheitslücke meldet, steht in `SECURITY.md` im Repo.

## Grundsätze

1. **Im Zweifel entscheidet der Server.** Clients rechnen mit Rollback selbst, aber sie schicken nur Absichten (`MageIntent`), nie Ergebnisse: keine Positionen, keine Treffer, keinen Schaden, kein Mana. Was ein Client lokal ausrechnet, ist nur für seine Anzeige; maßgeblich ist die Simulation des Servers ([[adr-0017-rollback-mit-server-als-schiedsrichter|ADR-0017]]).
2. **Nur senden, was gesehen werden darf.** Der Server leitet einem Client nur die Intents der Spieler weiter, die dessen Team sieht, und schickt nur Zustand von Entities, die es sieht. Eingaben und Zustand unsichtbarer Gegner verlassen den Server nie in Richtung ihrer Gegner ([[adr-0018-sichtbarkeit-als-simulationszustand|ADR-0018]]). Was nicht im Speicher des Clients ist, kann kein Wallhack anzeigen.
3. **Alles von außen ist feindlich.** Jede Nachricht wird begrenzt gelesen, geprüft und im Zweifel verworfen.
4. **Keine Geheimnisse im Repo, im Client oder in Logs.**
5. **Fremder Code läuft nur in der Sandbox** ([[adr-0025-add-ons-als-intent-produzenten|ADR-0025]]).

## Bedrohungen und Gegenmaßnahmen

| Bedrohung | Gegenmaßnahme |
| --- | --- |
| Speedhack, Teleport, Mana- oder Cooldown-Manipulation | Server simuliert selbst; der Client meldet nur Intents. `move_axis` wird auf Länge ≤ 1 geklemmt, `aim_dir` normalisiert, ungültige Werte (NaN, unendlich) verworfen. Cooldowns und Mana prüft das Casting-System des Servers. |
| Wallhack, Maphack, Unsichtbare sehen | Interest Management: Intents und Zustand unsichtbarer oder verdeckter Gegner werden gar nicht übertragen; wird jemand unsichtbar, endet die Weiterleitung seiner Intents im selben Tick. Auch keine indirekten Hinweise (Sounds, Ereignisse) außerhalb der Sichtbarkeit. |
| Aimbot, Automatisierung | Technisch nicht sicher erkennbar, weil der Client den `aim_dir` erzeugt. Gegenmaßnahmen: Plausibilitätsprüfung der Drehgeschwindigkeit auf dem Server, Statistik-Auffälligkeiten melden, Regeln pro Spielmodus für Add-ons ([[p-010-add-on-laufzeit-und-fairness\|P-010]]). Kein Versprechen, das wir nicht halten können. |
| Paket-Fälschung, Mitlesen, Man-in-the-Middle | Nur verschlüsselter Transport (TLS 1.3 über QUIC oder WebTransport; Details in [[p-009-netzwerk-crate-und-transport\|P-009]]). Server-Zertifikat wird geprüft. |
| Wiedereinspielen alter Pakete | Tick-Nummern und Sequenznummern pro Verbindung; alles außerhalb eines kleinen Fensters wird verworfen. |
| Fremde Verbindung übernehmen | Sitzungs-Token pro Verbindung, kurzlebig, an die Verbindung gebunden, nie in URLs oder Logs. Kontensystem erst nach M7; bis dahin Beitritts-Code pro Runde. |
| Überlastung (DoS), riesige oder viele Pakete | Größenobergrenze pro Nachricht und pro Sekunde und Verbindung; Verbindungen ohne gültigen Handshake werden nach kurzer Zeit getrennt; ein Intent pro Spieler und Tick, Überzählige werden verworfen. |
| Absturz durch fehlerhafte Daten | Begrenzte Parser: Längenpräfixe mit Obergrenze, keine unbegrenzten Listen, kein `unwrap` auf Netzwerkdaten. Fuzz-Tests für alle Nachrichtentypen. Ein fehlerhafter Client wird getrennt, der Server läuft weiter. |
| IP-Adressen von Mitspielern | Kein Peer-to-Peer, auch nicht für Rollback; Spieler sehen nur den Server. |
| Manipulierter Client-Zustand (eigene Simulation verfälscht) | Ohne Wirkung auf andere: Der Client schickt keinen Zustand, und der Server korrigiert ihn beim nächsten Prüfstand. |
| Bösartige Add-ons | WebAssembly-Sandbox ohne Dateisystem, Netzwerk und Uhr; Rechenzeit- und Speichergrenzen pro Tick ([[add-ons\|Add-ons]]). |
| Unsichere Abhängigkeiten | `Cargo.lock` eingecheckt; CI prüft bekannte Schwachstellen und Lizenzen der Abhängigkeiten (`cargo audit` oder `cargo deny`); neue Abhängigkeiten nur begründet. |
| Persönliche Daten | Server-Logs ohne IP-Adressen im Klartext und ohne Chat-Inhalte; Replays enthalten nur `PlayerId`s. Details mit dem Kontensystem. |

## Regeln für jede Änderung

Gelten für Menschen und Agenten, in jedem Review:

- Neue Nachrichtentypen: begrenzt lesbar, versioniert, mit Fuzz-Test, und hier in der Tabelle eingetragen.
- Nichts, was ein Client schickt, wird ungeprüft in den Simulationszustand übernommen.
- Nichts, was ein Team nicht sehen darf, verlässt den Server in Richtung dieses Teams, auch nicht in Fehlermeldungen oder Debug-Ausgaben.
- Keine Schlüssel, Tokens oder Passwörter im Repo, in Beispielen oder in Logs.
- Neue Abhängigkeiten: Plattformen ([[plattformen|Plattformen]]), Wartung, Lizenz und bekannte Schwachstellen prüfen.

## Was offen ist

- Transport und Verschlüsselung im Detail: [[p-009-netzwerk-crate-und-transport|P-009]]
- Fairness von Automatisierung und Zielhilfe: [[p-010-add-on-laufzeit-und-fairness|P-010]], [[p-011-eingabe-crate-und-zielhilfe|P-011]]
- Konten, Anmeldung, Matchmaking, Datenschutz: nach Milestone 7, eigene Seite.
