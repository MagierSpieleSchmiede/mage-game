---
title: "ADR-0031 — Magier und Kreaturen sind Einheiten mit gemeinsamem Kern; Bosse, Pets und Tiere sind Kreaturen"
---
# ADR-0031 — Magier und Kreaturen sind Einheiten mit gemeinsamem Kern; Bosse, Pets und Tiere sind Kreaturen

**Status:** angenommen (2026-11-15)

**Kontext:** Neben Spielern soll es Bosse, Pets, wilde Tiere und andere Gegnertypen geben, und Reittiere sollen eigenes Leben haben ([[adr-0029-reittiere-als-bewegungsmodus|ADR-0029]]). Bisher kennt die Simulation nur den Magier, gebunden an eine `PlayerId`. Vieles teilen alle diese Wesen (Team, Leben, Attribute, Fähigkeiten, Bewegung, Treffer), manches nicht: Kreaturen haben keinen Spieler, Bosse haben Phasen und mehrere Trefferzonen, Pets einen Besitzer.

**Entscheidung:**

1. Eine **Einheit** ist jede Simulations-Entity, die kämpfen kann. Ihr Kern: `SimId`, `TeamId`, Leben, Attribute, Fähigkeitenleiste, Kampfzustand ([[adr-0032-kampfzustand|ADR-0032]]), Bewegung per `MoveAndSlide`, ein Intent pro Tick und eine **Steuerung** (`Controller`): Spieler (`PlayerId`) oder KI (Verhaltens-ID).
2. Ein **Magier** ist eine Einheit mit Spieler-Steuerung. Eine **Kreatur** ist eine Einheit aus einem `CreatureBlueprint` (Werte, Fähigkeiten, Form, Verhalten), meist mit KI-Steuerung.
3. Besonderheiten sind zusätzliche Components, keine eigenen Entity-Arten: `Owner(SimId)` für Pets, `Rideable` für Reittiere, `BossPhases` und mehrere Trefferzonen für Bosse.
4. Alle Systeme für Kampf, Bewegung, Attribute und Sichtbarkeit fragen den Kern ab, nie „ist ein Magier“. Schaden, Tod und Respawn gehen für alle Einheiten durch dieselben Codepfade (Invariante 8).
5. Der Intent gehört zur gesteuerten Einheit: Der `InputBuffer` ist nach der `SimId` der Einheit geordnet; ein Spieler schreibt für die Einheit, an die seine `PlayerId` gebunden ist, eine KI für ihre Kreaturen.

**Verworfene Alternative:** Kreaturen als Magier mit einer Pseudo-`PlayerId`. Verworfen, weil Bosse, Pets und Reittiere dann Spieler-Konzepte (Konto, Verbindung, Leiste aus dem Profil) mitschleppen und eigene Konzepte (Besitzer, Phasen, Trefferzonen) nicht haben. Ebenso verworfen: getrennte Entity-Arten ohne gemeinsamen Kern, weil Kampf und Sichtbarkeit dann doppelt gebaut würden.

**Konsequenzen:** Was B-009 für den Magier baut (Leben, Mana, Cooldowns, Bewegung), gehört an den Kern. KI-Kreaturen brauchen die KI aus M6 ([[ki-und-rl|KI & RL]]), nutzen aber dieselbe [[beobachtung|Beobachtung]] und schummeln nicht. Viele Kreaturen in der Welt kosten Rechenzeit; KI entscheidet nur im Interessenbereich eines Spielers ([[welt|Welt]]). Details: [[einheiten|Einheiten]].
