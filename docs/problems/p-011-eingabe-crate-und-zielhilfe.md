---
title: "P-011 — Eingabe-Crate und Zielhilfe für Controller"
---
# P-011 — Eingabe-Crate und Zielhilfe für Controller

Eingabe läuft über Aktionen ([[adr-0022-eingabe-ueber-aktionen|ADR-0022]]). Offen: Bevys eigene Eingabe plus eine kleine eigene Belegungsschicht, oder eine fertige Crate für Aktions-Belegung (muss Bevy 0.20, Controller, Touch und Umbelegen können). Zweitens: Mit Controller zielt man ungenauer als mit Maus. Eine Zielhilfe würde im Client entstehen und den `aim_dir` verändern, was im PvP gegen Maus-Spieler unfair sein kann und sich technisch wie ein Aimbot verhält.

Nächster Schritt: in M3 die Crate-Frage entscheiden; die Zielhilfe-Frage vor M7 (Netzwerk) als ADR klären, zusammen mit den Add-on-Regeln ([[p-010-add-on-laufzeit-und-fairness|P-010]]).
