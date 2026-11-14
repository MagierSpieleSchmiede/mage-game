---
title: "P-010 — Add-on-Laufzeit und Fairness von Automatisierung"
---
# P-010 — Add-on-Laufzeit und Fairness von Automatisierung

Add-ons laufen als WebAssembly in einer Sandbox ([[adr-0025-add-ons-als-intent-produzenten|ADR-0025]]). Offen: welche Laufzeit. Eine JIT-Laufzeit ist schnell, ist aber auf iOS nicht erlaubt und im Browser nicht verfügbar; ein Interpreter läuft überall, ist aber langsamer. Möglich ist auch beides hinter einer gemeinsamen Schnittstelle. Zweitens: Automatisierung lässt sich serverseitig nicht sicher von menschlicher Eingabe unterscheiden. Ein modifizierter Client kann jedes Verbot umgehen. Welche Regeln gelten für öffentliche oder gewertete Spiele?

Nächster Schritt: vor M8 Laufzeit-Kandidaten auf allen Zielplattformen messen; Regeln je Spielmodus festlegen (Vorschlag: private Spiele frei, öffentliche nur Auslesen) und beides als ADR festhalten.
