---
title: "P-002 — Observer-Laufzeitpunkt innerhalb der Tick-Kette (entschieden)"
---
# P-002 — Observer-Laufzeitpunkt innerhalb der Tick-Kette (entschieden)

Entschieden durch [[adr-0014-impacts-sammeln-dann-sortiert-ausloesen|ADR-0014]]: Impacts werden im Set Treffer gesammelt und im Set Schaden sortiert ausgelöst. Nächster Schritt: in M2 mit einem Integrationstest festschreiben, dass die verschachtelten Trigger sofort durchlaufen, also ein tödlicher Treffer noch im selben Tick den `Dead`-Marker setzt und das Set Aufräumen ihn sieht.
