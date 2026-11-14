---
title: "P-009 — Netzwerk-Crate und Transport"
---
# P-009 — Netzwerk-Crate und Transport

Der Server-autoritative Ansatz ([[adr-0017-server-autoritativ-mit-client-vorhersage|ADR-0017]]) braucht einen Transport, der auf allen Plattformen funktioniert: UDP-basiert und verschlüsselt auf Desktop und Mobile, im Browser WebTransport (oder WebSocket als Rückfall). Offen ist, ob eine fertige Bevy-Netzwerk-Crate (mit Vorhersage, Interpolation, Interest Management) unsere Anforderungen erfüllt (eigene Tick-Kette, eigene Snapshots, Sichtbarkeitsfilter, deterministische Simulation) oder ob nur der Transport fertig übernommen und das Protokoll selbst gebaut wird.

Nächster Schritt: zu Beginn von M7 die aktuellen Kandidaten für Bevy 0.20 gegen [[netzwerk|Netzwerk]] und [[sicherheit|Sicherheit]] prüfen (Plattformen inklusive Browser, Verschlüsselung, Wartung, Lizenz) und als ADR entscheiden.
