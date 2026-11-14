---
title: "P-013 — Brücke zum Reinforcement Learning"
---
# P-013 — Brücke zum Reinforcement Learning

RL-Training läuft üblicherweise in Python. Offen ist, wie die Rust-Simulation dort ankommt: als Python-Modul (direkt eingebunden, schnell, mehrere Umgebungen pro Prozess) oder als eigener Prozess mit Nachrichtenprotokoll (sprachunabhängig, einfacher zu verteilen, langsamer). Dazu: welche Multi-Agent-Schnittstelle (für Teams), und wie trainierte Modelle zurück ins Spiel kommen (Export in ein Austauschformat und Ausführung mit einer reinen Rust-Inferenz-Bibliothek, damit es auch auf Mobilgeräten und im Browser läuft).

Nächster Schritt: zu Beginn von M6 einen kleinen Durchstich bauen (eine Umgebung, ein zufälliger Agent, Schritte pro Sekunde messen) und als ADR entscheiden.
