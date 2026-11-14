---
title: "ADR-0027 — Friendly Fire wird pro Fähigkeit über einen Zielfilter entschieden"
---
# ADR-0027 — Friendly Fire wird pro Fähigkeit über einen Zielfilter entschieden

**Status:** angenommen

**Kontext:** Mit Teams ([[adr-0015-teams-statt-fester-zwei-spieler|ADR-0015]]) musste feststehen, ob eigene Zauber Verbündete treffen ([[p-007-friendly-fire-und-team-regeln|P-007]]). Eine globale Regel passt nicht zu allen Fähigkeiten: Ein schneller Bolt soll durch Mitspieler fliegen, ein Feuerball darf ein Risiko für das eigene Team sein, ein späterer Heilzauber soll nur Verbündete treffen.

**Entscheidung:** Jeder Zauber-Blueprint trägt einen **Zielfilter** (`TargetFilter`: Gegner, Verbündete, Zaubernder; beliebig kombinierbar). Er gilt für die Trefferprüfung des Deliverys (nicht erfasste Magier werden durchflogen) und für alle on-Impact-Effekte; ein Effekt kann ihn weiter einschränken, nie erweitern. Hindernisse blockieren immer. Startwerte: Bolt und Homing-Bolt nur Gegner; Feuerball und Fragmente alle, auch Verbündete und der Zaubernde.

**Verworfene Alternative:** Ein globaler Schalter `friendly_fire` für die ganze Runde. Verworfen, weil er nicht zwischen Fähigkeiten unterscheiden kann und Heil- oder Unterstützungszauber gar nicht abbildet. Ein Spielmodus kann später zusätzlich alle Filter einschränken (etwa „kein Friendly Fire im Einsteiger-Modus“).

**Konsequenzen:** Der Filter ist Teil des Blueprints und damit Balance-Daten, die auch das RL-Balancing variieren kann. Trefferprüfung und Effekt-Handler lesen `TeamId` von Zaubernden und Getroffenen. Tests prüfen pro Fähigkeit, wen sie trifft und wen nicht. Details: [[zauber-datenmodell|Zauber-Datenmodell]].
