---
title: "ADR-0004 — Latch-Zähler für Flanken statt Vor-Tick-Ableitung allein"
---
# ADR-0004 — Latch-Zähler für Flanken statt Vor-Tick-Ableitung allein

**Kontext:** Ein Frame kann null oder mehrere fixe Ticks ausführen; ein kurzer Tastendruck darf weder verloren gehen noch doppelt zählen.
**Entscheidung:** Der Controller zählt Druckereignisse im `InputBuffer` (`press_count`); das Set Intent leitet daraus `pressed` im `MageIntent` ab und setzt den Zähler zurück. Der Zähler selbst gehört nicht in den Intent.
**Verworfene Alternative:** Flanken nur aus dem Vor-Tick-Intent ableiten. Verworfen, weil Drücken und Loslassen zwischen zwei Ticks unsichtbar blieben.
**Konsequenzen:** Der Intent bleibt reine Tick-Daten (drei zusätzliche bool) und ist ohne Änderung netzwerktauglich; das Protokoll ist in einem Satz dokumentierbar und mit Unit-Tests prüfbar.
