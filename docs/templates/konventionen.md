---
title: "Konventionen — wie dieser Vault geschrieben wird"
---
# Konventionen — wie dieser Vault geschrieben wird

## Seitenaufbau

Jede Seite: Titel nach dem Muster „Thema — der Punkt in einem Halbsatz“. Erst was gilt, dann warum, dann was fehlt. Keine Seite ohne Aussage über offene Fragen; zu jeder offenen Frage gehört ein Eintrag im [[problem-log|Problem-Log]]. Entscheidungen bekommen nie Prosa, sondern eine ADR.

Jede Seite beginnt mit Front Matter (`title:` in Anführungszeichen) und derselben Überschrift als `# Titel`. Verweise sind Obsidian-Links (`[[dateiname|Anzeigetext]]`).

## Entscheidungen und Probleme

- **ADR:** eine Seite `decisions/adr-nnnn-<slug>.md` nach der [[adr-nnnn|ADR-Vorlage]]. Eine angenommene ADR wird nicht still geändert; wer sie kippen will, schreibt eine neue ADR, die sie ersetzt. Jede ADR hat einen **Status**: *vorgeschlagen* (wartet auf Daniels Freigabe), *angenommen*, oder *ersetzt durch ADR-nnnn*. Nächste freie Nummer: **ADR-0028**.
- **Problem:** eine Seite `problems/p-nnn-<slug>.md` nach der [[p-nnn|P-Vorlage]] und eine Zeile im [[problem-log|Problem-Log]]. Nächste freie Nummer: **P-016**.
- **Backlog-Item:** eine Zeile im [[backlog|Backlog]] (`B-nnn`) mit Abhängigkeiten und den zuerst zu schreibenden Tests; erledigte Items werden entfernt und ins Implementation-Log übernommen.
- **Implementation-Log:** pro Milestone eine Seite `milestones/mN-implementation-log.md` („Milestone N — Implementation Log“), sobald gebaut wird: was gebaut wurde, was anders kam als geplant, welche ADRs und P-Einträge dabei entstanden.

## Ablage

| Inhalt | Ordner und Datei |
| --- | --- |
| Startseite | `index.md` |
| Backlog | `backlog.md` |
| Status | `status.md` |
| Architektur | `architecture/<slug>.md` |
| Entscheidungen | `decisions/adr-nnnn-<slug>.md` |
| Spezifikationen | `specifications/spec-nnn-<slug>.md` |
| Testing | `testing/<slug>.md` |
| Roadmap, Milestones, Erweiterungspunkte | `milestones/<slug>.md` |
| Probleme | `problems/p-nnn-<slug>.md` plus `problems/problem-log.md` |
| Konventionen und Vorlagen | `templates/` |
| Glossar | `glossar.md` |
| Gestaltung (Entwürfe, Moodboards) | `gestaltung/<thema>.md`, Bilder in `gestaltung/bilder/` |
| Einstiegsanleitungen | `einstieg-<thema>.md` |
