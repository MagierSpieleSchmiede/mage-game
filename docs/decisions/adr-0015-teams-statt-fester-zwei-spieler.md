---
title: "ADR-0015 — Teams mit beliebig vielen Magiern statt eines festen Zwei-Spieler-Duells"
---
# ADR-0015 — Teams mit beliebig vielen Magiern statt eines festen Zwei-Spieler-Duells

**Status:** angenommen (2026-11-14)

**Kontext:** Das Spiel soll nicht nur 1-gegen-1 laufen, sondern mit zwei oder mehr Teams aus jeweils mehreren Spielern. Der ursprüngliche Entwurf kannte genau zwei Magier; Homing suchte „den“ Gegner, das HUD zeigte „beide“ Magier.

**Entscheidung:** Jeder Magier trägt eine `TeamId` (u8). Die Simulation kennt beliebig viele Magier in beliebig vielen Teams (Obergrenze 32 Teams wegen der Sichtbarkeits-Bitmaske, [[adr-0018-sichtbarkeit-als-simulationszustand|ADR-0018]]). Jede Regel, die „Gegner“ sagt, meint „Magier eines anderen Teams“. Zielwahl (Homing) und Flächenschaden fragen das Team ab, nie eine feste Gegner-ID. Der Prototyp testet 1-gegen-1 und 2-gegen-2.

**Verworfene Alternative:** Zwei feste Spieler jetzt, Teams später nachrüsten. Verworfen, weil „der Gegner“ sich sonst in Zielwahl, HUD, Sichtbarkeit, Netzwerk und KI festsetzt und jede Stelle später angefasst werden müsste.

**Konsequenzen:** Zielwahl braucht eine deterministische Regel (nächster sichtbarer Gegner, Gleichstand nach `SimId`). Ob Zauber das eigene Team treffen, entscheidet jede Fähigkeit über ihren Zielfilter ([[adr-0027-zielfilter-pro-faehigkeit|ADR-0027]]). Kosten der Sichtbarkeits- und Trefferprüfung wachsen mit der Spielerzahl und werden gemessen ([[benchmarks|Benchmarks]]).
