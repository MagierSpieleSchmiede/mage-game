---
title: "P-022 — Wer ist wem feind, wenn es wilde Tiere, neutrale Kreaturen und Pets gibt?"
---
# P-022 — Wer ist wem feind, wenn es wilde Tiere, neutrale Kreaturen und Pets gibt?

Bisher gilt: Feind ist, wer ein anderes Team hat ([[adr-0015-teams-statt-fester-zwei-spieler|ADR-0015]]). Mit Kreaturen ([[einheiten|Einheiten]]) reicht das nicht: Wilde Tiere greifen alle an, neutrale Tiere nur, wenn man sie angreift, Pets gehören zum Team ihres Besitzers. Möglichkeit: eine Beziehungstabelle zwischen Teams (feindlich, neutral, verbündet) in der Balance, dazu „neutral wird feindlich, wenn angegriffen“ als Zustand der Einheit. Betrifft Zielfilter ([[adr-0027-zielfilter-pro-faehigkeit|ADR-0027]]), Homing, Kampfzustand und Sichtbarkeit ([[p-018-sichtbarkeit-bei-vielen-teams|P-018]]).

Nächster Schritt: mit den ersten Kreaturen (nach M6) entscheiden, als ADR; bis dahin bleibt „anderes Team = Feind“.
