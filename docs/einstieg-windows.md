---
title: "Mage Game – Einstieg für Mitwirkende (Windows)"
---
# Mage Game – Einstieg für Mitwirkende (Windows)

Nov 14, 2026 · @Daniel

Diese Anleitung bringt dich von null bis zum ersten eigenen Beitrag. Danach geht es je nach Interesse weiter mit [[einstieg-ki-agenten|KI-Agenten (Claude Code, Codex)]] oder [[einstieg-gestalten|Gestalten (Blender, ArtCraft)]]. Du brauchst keine Vorkenntnisse – nur einen Windows-PC, etwa eine Stunde Zeit und ein bisschen Geduld beim ersten Kompilieren.

## 1. Worum es geht

**Mage Game** ist ein 3D-Arena-Spiel in Rust: Teams von Magiern, freie Bewegung, Third-Person-Zielen, modulare Zauber (Bolt, Homing-Bolt, Feuerball). Alles, was wir planen und entscheiden, steht im **Vault** – einer Sammlung von Markdown-Seiten, die du mit Obsidian liest. Der Code und der Vault liegen zusammen in **einem** GitHub-Repository.

## 2. Die Werkzeuge auf einen Blick

| Werkzeug | Wofür | Muss ich das haben? |
| --- | --- | --- |
| GitHub-Konto | Zugang zum gemeinsamen Projekt | Ja |
| Git | Versionsverwaltung auf deinem PC | Ja |
| GitHub Desktop | Git mit Knöpfen statt Befehlen | Empfohlen für Einsteiger |
| Visual Studio Build Tools | C++-Linker, den Rust unter Windows braucht | Ja (für Code) |
| Rust (rustup) | Compiler und Paketverwaltung `cargo` | Ja (für Code) |
| VS Code + rust-analyzer | Code-Editor mit Rust-Unterstützung | Empfohlen |
| Obsidian | Vault lesen und bearbeiten | Ja |
| Claude Code oder Codex | KI-Agent, der beim Programmieren und Schreiben hilft | Optional, siehe [[einstieg-ki-agenten\|KI-Agenten]] |
| Blender, ArtCraft | Modelle, Konzeptbilder, Look des Spiels | Optional, siehe [[einstieg-gestalten\|Gestalten]] |

Wer nur am Vault (Konzept, Balance, Ideen) mitarbeitet, braucht Build Tools, Rust und VS Code nicht. Wer gestalten will, liest danach [[einstieg-gestalten|Gestalten]]; wer mit einem KI-Agenten arbeiten will, [[einstieg-ki-agenten|KI-Agenten]].

## 3. Was sind Git und GitHub?

**Git** ist ein Programm, das sich jede Version jeder Datei merkt. Statt `feuerball_final_v3_WIRKLICH.txt` gibt es eine saubere Geschichte: wer hat wann was geändert und warum. Nichts geht verloren, alles lässt sich zurückdrehen.

**GitHub** ist die Website, auf der unser Projekt zentral liegt – wie ein gemeinsamer Ordner in der Cloud, nur dass Änderungen nicht einfach überschrieben werden, sondern bewusst zusammengeführt.

Der wichtigste Unterschied zu Dropbox oder Nextcloud: Dort wird alles automatisch synchronisiert, und wenn zwei Leute gleichzeitig dieselbe Datei ändern, gewinnt einer. Bei Git entscheidest du selbst, wann du einen Stand abschickst, und andere können ihn sich vorher ansehen.

Die Begriffe, die du ständig hören wirst:

- **Repository (Repo):** das Projekt mit seiner ganzen Geschichte.
- **Clone:** das Repo einmalig auf deinen PC holen.
- **Commit:** ein gespeicherter Schnappschuss deiner Änderungen, mit kurzer Beschreibung.
- **Branch:** ein eigener Arbeitszweig, auf dem du ungestört etwas ausprobierst, ohne `master` (den Hauptstand) zu verändern.
- **Push:** deine Commits zu GitHub hochladen.
- **Pull:** die Änderungen der anderen herunterladen.
- **Pull Request (PR):** die Bitte „bitte übernehmt meinen Branch in `master`“. Hier wird geschaut, kommentiert und dann zusammengeführt (**Merge**).
- **Merge-Konflikt:** zwei Leute haben dieselbe Stelle geändert; Git fragt, welche Version gelten soll.

## 4. Einmalige Einrichtung

### 4.1 GitHub-Konto

1. Auf github.com ein kostenloses Konto anlegen.
2. Zwei-Faktor-Authentifizierung einschalten (GitHub verlangt das ohnehin bald).
3. Mir deinen **GitHub-Benutzernamen** schicken. Ich lade dich ins Repo ein; die Einladung kommt per E-Mail und muss angenommen werden.

### 4.2 Programme installieren

Am schnellsten geht es mit `winget`, das in Windows 10/11 schon eingebaut ist. Startmenü → „PowerShell“ öffnen und diese Zeilen nacheinander ausführen:

```powershell
winget install --id Git.Git -e
winget install --id GitHub.GitHubDesktop -e
winget install --id Obsidian.Obsidian -e
winget install --id Microsoft.VisualStudioCode -e
```

Nur wer programmiert, zusätzlich:

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override "--wait --passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
winget install --id Rustlang.Rustup -e
```

Die Build Tools sind groß (mehrere GB) und brauchen eine Weile. Danach PowerShell **schließen und neu öffnen**, damit die neuen Programme gefunden werden. Prüfen:

```powershell
git --version
cargo --version
```

In VS Code links auf das Erweiterungen-Symbol klicken und **rust-analyzer** installieren.

### 4.3 Git sagen, wer du bist

Einmalig, damit deine Commits deinen Namen tragen. Die E-Mail sollte die deines GitHub-Kontos sein (oder die anonyme `…@users.noreply.github.com` aus den GitHub-E-Mail-Einstellungen):

```powershell
git config --global user.name "Dein Name"
git config --global user.email "deine@mail.de"
```

In GitHub Desktop: *File → Options → Accounts* → bei GitHub anmelden. Damit ist auch der Zugang zum Repo erledigt, ohne dass du Passwörter oder Schlüssel einrichten musst.

### 4.4 Das Projekt holen (Clone)

1. GitHub Desktop → *File → Clone repository*.
2. Im Reiter *GitHub.com* das Repo `mage-game` auswählen.
3. Als Ort einen einfachen Pfad ohne Leer- und Sonderzeichen wählen, z. B. `C:\dev\mage-game`.

> **Wichtig:** Den Projektordner **nicht** in OneDrive, Dropbox, Nextcloud oder Google Drive legen. Diese Dienste synchronisieren Git-interne Dateien und den riesigen Build-Ordner `target`, was Repos beschädigen und den PC lahmlegen kann.

### 4.5 Erster Start

Im Projektordner (GitHub Desktop → *Repository → Open in Command Prompt*):

```powershell
cargo run
```

Der **erste** Build dauert lange – je nach PC 5 bis 20 Minuten, weil die Spiel-Engine Bevy komplett kompiliert wird. Danach geht es in Sekunden. Tests laufen mit:

```powershell
cargo test
```

### 4.6 Den Vault öffnen

Obsidian starten → *Open folder as vault* → den Ordner `docs` im Projekt wählen. Fertig: Jede Seite ist eine normale `.md`-Datei im Repo, und Änderungen daran laufen genauso über Git wie Code.

Zum reinen Lesen gibt es den Vault auch als Webseite: <https://magierspieleschmiede.github.io/mage-game/>. Sie wird nach jedem Push auf `master` automatisch neu gebaut.

## 5. Der Arbeitsablauf – jedes Mal

> **Solange wir planen** (vor Milestone 1, siehe [[status|Status]]), arbeiten alle direkt auf `master`: pullen, ändern, committen, pushen. Kleine Commits, vor jedem Start pullen. Branches und Pull Requests wie unten beschrieben gelten **ab Beginn der Implementierung**; das wird im Chat angesagt.

Ab der Implementierung:

1. **Aktualisieren:** In GitHub Desktop auf `master` wechseln und *Fetch origin* → *Pull origin* klicken.
2. **Branch anlegen:** *Current branch → New branch*, sprechender Name wie `feuerball-split` oder `vault-balance-mana`.
3. **Arbeiten:** Code in VS Code, Vault in Obsidian ändern.
4. **Committen:** GitHub Desktop zeigt alle Änderungen. Unten links eine kurze Beschreibung („Split-Fragmente parallel zur Fläche“) eintragen → *Commit*. Lieber oft kleine Commits als einen riesigen.
5. **Pushen:** *Publish branch* bzw. *Push origin*.
6. **Pull Request:** *Create Pull Request* öffnet GitHub im Browser. Kurz beschreiben, was und warum, dann abschicken.
7. **Review & Merge:** Jemand schaut drüber, kommentiert, und wenn alles passt, wird gemergt. Danach den Branch löschen und wieder bei Schritt 1 anfangen.

**Die drei goldenen Regeln:**

- Ab der Implementierung nie direkt auf `master` arbeiten – immer ein eigener Branch.
- Vor dem Start immer erst pullen.
- Bevor du Code abschickst: `cargo test` muss grün sein.

## 6. Wer macht was, wo wird gesprochen

- **Aufgaben** stehen als *Issues* auf GitHub. Wer eine übernimmt, weist sie sich selbst zu, damit niemand doppelt arbeitet.
- **Entscheidungen** (ADRs) und **offene Probleme** (P-Log) stehen im Vault. Eine neue Entscheidung ist eine neue ADR-Seite (nach der [[adr-nnnn|ADR-Vorlage]]).
- **Kommentare zu einer Vault-Seite** direkt unten auf der Seite der Webseite (Anmeldung mit GitHub). Sie landen in den *Discussions* des Repos.
- **Schnelle Absprachen** im Chat; was dabei beschlossen wird, gehört danach in den Vault oder ein Issue, sonst ist es vergessen.

## 7. Wenn etwas schiefgeht

| Problem | Lösung |
| --- | --- |
| `linker 'link.exe' not found` | Die Visual Studio Build Tools fehlen oder ohne C++-Workload. Schritt 4.2 wiederholen. |
| `cargo` oder `git` unbekannt | PowerShell neu öffnen; notfalls PC neu starten. |
| Push wird abgelehnt (*rejected*) | Erst *Pull* machen, dann erneut pushen. |
| Merge-Konflikt | Keine Panik. GitHub Desktop zeigt die betroffene Datei; in VS Code für jede Stelle „Accept Current“, „Accept Incoming“ oder beides wählen, speichern, committen. Im Zweifel fragen. |
| Versehentlich auf `master` gearbeitet | Nicht committen. *Current branch → New branch* – deine Änderungen wandern mit. |
| Alles kaputt, ich will zurück | Datei in GitHub Desktop rechtsklicken → *Discard changes*. Was committet ist, geht nicht verloren. |
| Erster Build hängt ewig | Normal. Beim zweiten Mal ist es schnell. |
| Festplatte voll, Ordner `target` riesig | `target` enthält nur Zwischenstände des Compilers und wächst mit jedem Versionswechsel von Bevy. Im Projektordner `cargo clean` ausführen: löscht ihn komplett, der nächste Build dauert dann wieder lange. Lohnt sich ab etwa 20 GB oder nach einem Update der Abhängigkeiten. Größe prüfen: Rechtsklick auf `target` → *Eigenschaften*. |

## 8. Checkliste

- [ ] GitHub-Konto angelegt, 2FA an, Benutzername geschickt
- [ ] Einladung zum Repo angenommen
- [ ] Git, GitHub Desktop, Obsidian installiert
- [ ] (Code) Build Tools, Rust, VS Code + rust-analyzer installiert
- [ ] Name und E-Mail in Git gesetzt
- [ ] Repo nach `C:\dev\mage-game` geklont (nicht in OneDrive!)
- [ ] `cargo run` startet das Spiel
- [ ] Vault in Obsidian geöffnet
- [ ] Ersten Commit gemacht und gepusht – zum Üben gern einen Tippfehler im Vault korrigieren
- [ ] (Optional) [[einstieg-ki-agenten|KI-Agent]] oder [[einstieg-gestalten|Gestaltungswerkzeuge]] eingerichtet
