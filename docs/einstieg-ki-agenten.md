---
title: "KI-Agenten — mit Claude Code oder Codex am Projekt arbeiten"
---
# KI-Agenten — mit Claude Code oder Codex am Projekt arbeiten

Ein Code-Agent ist ein KI-Assistent, der im Projektordner arbeitet: Er liest Dateien, schlägt Änderungen vor, schreibt Code und Vault-Seiten und führt Befehle wie `cargo test` aus. Bei uns sind zwei verbreitet: **Claude Code** (Anthropic) und **Codex** (OpenAI). Welchen du nimmst, hängt davon ab, welches Abo du hast. Beide halten sich an dieselben Projektregeln, weil beide die Datei `AGENTS.md` im Repo lesen.

Voraussetzung: Die Einrichtung aus [[einstieg-windows|Einstieg für Mitwirkende (Windows)]] ist erledigt, das Repo liegt z. B. unter `C:\dev\mage-game`.

## 1. Welcher Agent?

| | Claude Code | Codex |
| --- | --- | --- |
| Anbieter | Anthropic | OpenAI |
| Abo | Claude Pro, Max, Team oder Enterprise (der kostenlose Plan reicht nicht) | ChatGPT-Abo mit Codex-Zugang (aktuelle Pläne auf der Codex-Seite prüfen) |
| Oberflächen | Terminal, Desktop-App, VS-Code-Erweiterung | Terminal, Desktop-App, VS-Code-Erweiterung |
| Projektregeln | `CLAUDE.md` → verweist auf `AGENTS.md` | `AGENTS.md` direkt |

Wer schon ein Abo hat, nimmt den passenden Agenten. Für Einsteiger ist die **Desktop-App** oder die **VS-Code-Erweiterung** am angenehmsten; das Terminal kann man später lernen.

## 2. Installieren

### Claude Code

PowerShell öffnen und **eine** der beiden Varianten ausführen:

```powershell
irm https://claude.ai/install.ps1 | iex     # aktualisiert sich selbst
winget install Anthropic.ClaudeCode         # alternativ; Updates mit winget upgrade
```

PowerShell neu öffnen, prüfen mit `claude --version`. Git für Windows (aus der Grundeinrichtung) sollte installiert sein, dann kann Claude Code die Git-Bash nutzen. Wer lieber klickt: die Desktop-App von <https://claude.com/download>. Anleitung: <https://code.claude.com/docs/en/setup>.

### Codex

Die Desktop-App für Windows und den aktuellen Installationsbefehl für die Kommandozeile gibt es auf <https://developers.openai.com/codex>. Ein üblicher Weg für die Kommandozeile ist npm (braucht Node.js 22 oder neuer):

```powershell
npm install -g @openai/codex
```

Prüfen mit `codex --version`. Achtung: Das Paket heißt `@openai/codex`, nicht `codex`.

## 3. Erste Sitzung

1. Im Projektordner ein Terminal öffnen (GitHub Desktop → *Repository → Open in Command Prompt*) und `claude` bzw. `codex` starten. In der Desktop-App stattdessen den Ordner `C:\dev\mage-game` öffnen.
2. Beim ersten Start im Browser anmelden.
3. Zum Kennenlernen etwas fragen, das nichts verändert:

> Lies AGENTS.md und docs/status.md und erkläre mir in fünf Sätzen, wo das Projekt steht und was als Nächstes dran ist.

Der Agent fragt nach, bevor er Dateien ändert oder Befehle ausführt. **Lies, was er vorhat, bevor du zustimmst.**

## 4. So arbeiten wir mit Agenten

- **Du bist verantwortlich, nicht der Agent.** Alles, was du committest, hast du gelesen. Ein Agent kann sich irren, auch wenn er sehr sicher klingt.
- **Erst planen lassen, dann bauen.** Bitte den Agenten zuerst um einen Plan („noch nichts ändern“), prüfe ihn, dann gib ihn frei.
- **Klein halten.** Eine Aufgabe pro Sitzung: eine Vault-Seite, ein Modul, ein Test. Große Aufträge („bau das Spiel“) gehen schief.
- **Prüfen lassen.** Vor jedem Code-Commit muss der Agent die Gates aus `AGENTS.md` laufen lassen (`cargo fmt --check`, `cargo clippy`, `cargo test`), und du schaust dir das Ergebnis an.
- **Entscheidungen gehören in den Vault.** Schlägt der Agent eine neue Lösung vor, lass ihn eine ADR nach der [[adr-nnnn|ADR-Vorlage]] schreiben, statt sie still einzubauen. Offene Fragen kommen ins [[problem-log|Problem-Log]].
- **Keine Geheimnisse.** Keine Passwörter, Tokens oder privaten Daten in den Chat oder in Dateien geben.
- **Speicherplatz im Blick.** Agenten kompilieren oft. Der Ordner `target` kann dabei auf viele Gigabyte wachsen; wie man ihn aufräumt, steht in `AGENTS.md` und in der Fehlertabelle von [[einstieg-windows|Einstieg für Mitwirkende]].

Gute Aufträge sind konkret und nennen die Quelle:

> Lies docs/architecture/intent-steuerung.md. Schreib Unit-Tests für das Flanken-Protokoll, wie in docs/testing/teststrategie.md beschrieben. Noch keinen anderen Code ändern.

> Ich möchte einen Eiszauber vorschlagen, der Gegner verlangsamt. Prüfe gegen docs/architecture/zauber-datenmodell.md, ob er sich aus vorhandenen Bausteinen bauen lässt, und schreib den Vorschlag als neue Seite. Noch nicht committen.

## 5. Wenn etwas schiefgeht

| Problem | Lösung |
| --- | --- |
| `claude` oder `codex` unbekannt | PowerShell neu öffnen; Installation prüfen. |
| Agent hat Unsinn geändert | In GitHub Desktop die Datei rechtsklicken → *Discard changes*. Nichts ist verloren, solange nicht committet ist. |
| Agent will etwas tun, das du nicht verstehst | Ablehnen und nachfragen: „Erklär mir, was dieser Befehl macht.“ |
| Festplatte läuft voll | Ordner `target` aufräumen, siehe Fehlertabelle in [[einstieg-windows\|Einstieg für Mitwirkende]]. |
