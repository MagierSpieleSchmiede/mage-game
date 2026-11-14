---
title: "ADR-0026 — Tests zuerst: jede Funktion beginnt mit einem fehlschlagenden Test"
---
# ADR-0026 — Tests zuerst: jede Funktion beginnt mit einem fehlschlagenden Test

**Status:** angenommen (2026-11-14, mit Daniels Start der Implementierung)

**Kontext:** Ein großer Teil des Codes wird von KI-Agenten geschrieben. Ohne vorher festgelegte Erwartung lässt sich schwer prüfen, ob Code tut, was er soll, und Agenten neigen dazu, Tests an den Code anzupassen statt umgekehrt.

**Entscheidung:** Jedes Backlog-Item wird **test-first** umgesetzt: Zuerst die Tests schreiben, die das Verhalten aus dem Vault festhalten, sie laufen lassen und sehen, dass sie fehlschlagen; dann implementieren, bis sie grün sind; dann aufräumen. Tests werden nicht abgeschwächt, um Code durchzubringen; ist ein Test falsch, wird das begründet und im Vault nachgezogen. Neben Unit- und Integrationstests gehören Synchronisations-, Latenz- und Leistungstests dazu ([[teststrategie|Teststrategie]]).

**Verworfene Alternative:** Tests nach der Implementierung. Verworfen, weil die Tests dann den Code beschreiben statt die Anforderung.

**Konsequenzen:** Jedes Item im [[backlog|Backlog]] nennt seine Tests. Ein Commit darf rote Tests nur enthalten, wenn sie als `#[ignore]` mit Verweis auf ihr Backlog-Item markiert sind.
