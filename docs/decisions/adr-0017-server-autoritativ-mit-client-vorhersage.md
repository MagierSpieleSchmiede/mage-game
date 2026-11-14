---
title: "ADR-0017 — Server-autoritatives Netzwerk mit Client-Vorhersage statt Peer-to-Peer-Rollback"
---
# ADR-0017 — Server-autoritatives Netzwerk mit Client-Vorhersage statt Peer-to-Peer-Rollback

**Status:** vorgeschlagen

**Kontext:** Ursprünglich war PvP als Rollback-Netcode geplant, bei dem jeder Client den vollständigen Zustand simuliert. Neue Anforderungen widersprechen dem: Unsichtbarkeit (ein Client, der alles simuliert, kennt die Position unsichtbarer Gegner, ein Wallhack ist trivial), mehrere Teams mit vielen Spielern (Peer-to-Peer skaliert schlecht und verrät IP-Adressen), Add-ons, die den Zustand auslesen, und Clients auf verschiedenen Plattformen (plattformübergreifende Bit-Gleichheit ist nicht garantierbar, [[p-003-cross-plattform-determinismus-fuer-echtes-pvp|P-003]]).

**Entscheidung:** Ein **Server** (dediziert oder als Host im Spiel) führt die einzige maßgebliche Simulation. Clients schicken nur `MageIntent` pro Tick. Der Server schickt jedem Client nur, was dessen Team sehen darf (Interest Management, [[adr-0018-sichtbarkeit-als-simulationszustand|ADR-0018]]). Clients **sagen ihren eigenen Magier voraus** und gleichen bei jedem Server-Stand ab (Snapshot zurückspielen, eigene ausstehende Intents neu simulieren); fremde Entities werden interpoliert. Die vorhandene Snapshot-/Restore-Planung dient genau diesem Abgleich.

**Verworfene Alternative:** Peer-to-Peer-Rollback (jeder simuliert alles). Verworfen wegen Wallhack bei Unsichtbarkeit, IP-Leaks, Skalierung mit vielen Spielern und Abhängigkeit von plattformübergreifender Bit-Gleichheit.

**Konsequenzen:** Es braucht einen Server-Prozess (dieselbe Simulations-Crate, headless). Determinismus bleibt wichtig (Vorhersage, Replays, Tests, RL), muss aber nicht mehr plattformübergreifend bit-genau sein: Abweichungen führen zu Korrekturen, nicht zu Desyncs. Sicherheit: [[sicherheit|Sicherheit]]; Details: [[netzwerk|Netzwerk]].
