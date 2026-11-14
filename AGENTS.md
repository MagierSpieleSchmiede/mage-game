# AGENTS.md — hier anfangen

Du arbeitest an **Mage Game**, einem stilisierten 3D-Arena-Spiel in Rust mit Bevy 0.20 und Avian3D 0.8: zwei oder mehr Teams von Magiern, freie 3D-Bewegung, Third-Person-Zielen, modulare Zauber (Bolt, Homing-Bolt, Feuerball), Attribute, die sich im Kampf ändern. Das gesamte Gameplay ist eine deterministische, headless-fähige Tick-Simulation im `FixedUpdate` (Crate `mage_sim`); die Präsentation ist ein reiner Beobachter. Ziele: Windows, Linux, macOS, Android, iOS (Web wenn machbar), Englisch und Deutsch, Tastatur/Maus und Controller. Später kommen Unsichtbarkeit, KI-Gegner und Reinforcement Learning, PvP über einen Server und Add-ons; dafür wird entworfen, aber noch nicht gebaut.

`docs/` ist der **Vault**, das Design-Gedächtnis des Projekts (ein Obsidian-Vault, als Webseite veröffentlicht). Lies ihn, bevor du etwas änderst, und pflege ihn, wenn du es tust.

## Stand

Der aktuelle Planungsstand steht an **einer** Stelle: `docs/status.md` (was existiert, was blockiert, was als Nächstes kommt). Diese Datei wiederholt ihn nicht. Zwei Regeln gelten unabhängig davon:

- Fang nicht ungefragt mit Code an; Daniel gibt den Start der Implementierung frei.
- Welche ADRs angenommen und welche nur *vorgeschlagen* sind, steht in `docs/decisions/entscheidungen.md`. Vorgeschlagene ADRs sind Planungsgrundlage; frag nach, bevor du Code darauf baust.

## Zuerst lesen

1. `docs/status.md`: der Planungsstand und der nächste Schritt. Wo der Code widerspricht, gewinnt der Code; dann die Seite korrigieren.
2. `docs/backlog.md`: **hier wählst du deine Aufgabe**, und oben steht der Ablauf, nach dem jedes Item abgearbeitet wird.
3. `docs/index.md`: die Inhaltskarte des Vaults, mit der Leseliste „Hier anfangen“.
4. `docs/architecture/simulationsmodell.md`: die Invarianten und die Tick-Kette.
5. `docs/specifications/spec-001-prototype.md`: Toolchain, Umfang, Anforderungen, Abnahmekriterien.
6. `docs/decisions/entscheidungen.md`: alle ADRs mit Stand. Was angenommen ist, gilt; eine angenommene ADR nicht neu verhandeln ohne neue ADR.
7. `docs/problems/problem-log.md`: offene Fragen mit ihrem nächsten Schritt.
8. `docs/architecture/sicherheit.md`: Sicherheitsregeln für jede Änderung, die Netzwerk, Eingaben von außen oder Add-ons berührt.
9. `docs/templates/konventionen.md`: wie Seiten, ADRs und P-Einträge geschrieben werden, und welche Tatsache wo steht („Eine Quelle pro Tatsache“).

Die Einstiegsseiten für Menschen (`docs/einstieg-windows.md`, `docs/einstieg-ki-agenten.md`, `docs/einstieg-gestalten.md`): Wer Arbeitsabläufe oder Regeln ändert, zieht sie mit.

## So arbeitest du ein Backlog-Item ab

Der Ablauf steht oben in `docs/backlog.md` und gilt für Menschen und Agenten gleich: Tests zuerst, Gates grün, Vault und Status nachziehen, Backlog pflegen. Danach `target/` prüfen (unten).

## Vor dem Commit: die Gates

```sh
cargo fmt --check
cargo check
cargo clippy --all-targets -- -D warnings
cargo test
```

Laut Spec 001 nach **jedem** Modul `cargo check` und `cargo clippy -- -D warnings`, beides sauber, bevor es weitergeht. Wer `docs/` oder `site/` ändert, prüft zusätzlich, dass `site/build.sh` durchläuft.

## Regeln

- **Keine Bevy-API aus dem Gedächtnis.** Bevy-Wissen von vor 0.20 ist nicht maßgeblich. Vor dem ersten Code die Migrationsleitfäden 0.18→0.19 und 0.19→0.20 lesen; jede unsichere API in der echten Crate-Doku oder im Quellcode (`~/.cargo/registry/src/`) prüfen. Dasselbe gilt für Avian 0.8.
- **Required Components** und einfache `commands.spawn((...))`-Tupel. Keine Legacy-Bundles, kein `bsn!`.
- **Die Simulations-Invarianten halten**, insbesondere: kein Gameplay-Zustand in `Local<>`; keine Frame-Zeit in Simulationscode, alle Dauern in Ticks; Zufall nur aus der einen geseedeten RNG-Ressource; Reihenfolge nach `SimId`, nie nach `Entity`; Simulations-Entities tragen keine Meshes, Materialien, Lichter oder UI; Schaden, Mana und Tod haben je genau einen Codepfad.
- **Die Präsentation schreibt genau eine Sache zurück: den Intent.** Die Simulation liest niemals Präsentationszustand.
- **Lesen nur über die Beobachtung.** HUD, KI, Add-ons und Netzwerk-Versand bekommen ihre Daten aus der einen Beobachtungsfunktion mit Sichtbarkeitsfilter, nie direkt aus der ECS-Welt.
- **Teams, nicht „der Gegner“.** Kein Code nimmt genau zwei Spieler an.
- **Animation blockiert nie eine Fähigkeit.** Wann etwas wirkt, entscheidet die Simulation nach Ticks; keine Animations-Events oder Root Motion im Gameplay.
- **Keine sichtbaren Texte im Code.** Alles über Fluent-Schlüssel, in Englisch und Deutsch.
- **Keine Tasten im Code.** Eingabe über Aktionen; Tastatur/Maus und Controller gleichwertig.
- **Plattformen:** Nichts einbauen, was Windows, Linux, macOS, Android oder iOS ausschließt; `mage_sim` muss für `wasm32-unknown-unknown` bauen (keine Threads, kein Dateisystem, keine Systemzeit). Bei jeder neuen Abhängigkeit die Plattformen prüfen (`docs/architecture/plattformen.md`).
- **Leistung:** Wer Simulation oder Rendering ändert, vergleicht die Benchmarks vorher und nachher (`docs/testing/benchmarks.md`).
- **Balance-Konstanten** gehören in die zentrale Balance-Ressource, nie als magische Zahlen in Systeme.
- **Umfang:** Keine Abstraktion, die nicht der „Entwirf für“-Liste dient oder mindestens zweimal verwendet wird. Keine TODO-Stubs für Kernfunktionalität. Netzwerk, KI, Add-ons, Menüs, Audio, Asset-Pipelines, Spielstände und Animationen aus Asset-Dateien sind außerhalb des Prototyps.
- **`Cargo.lock` wird eingecheckt.** Neue Abhängigkeiten nur mit Begründung (ADR, wenn es eine Architekturfrage ist).
- **Branches und `master`:** Solange geplant wird (vor Milestone 1), wird direkt auf `master` committet und gepusht, wenn Daniel es sagt. Ab Beginn der Implementierung: eigener Branch, Pull Request; gemergt wird, wenn Daniel es sagt. Pushen immer nur nach Freigabe.
- **Nie committen:** `target/`, `site/.quartz/`, `site/public/`, `.env`, `.secrets`.

## `target/` klein halten

Bevy-Builds sind groß: Ein einziger Debug-Build legt mehrere Gigabyte in `target/` ab, und jeder Versionswechsel einer Abhängigkeit, jedes zusätzliche Profil (`--release`, `test`, `clippy`) und jedes zusätzliche Target-Verzeichnis legt weitere daneben. Alte Stände räumt Cargo nie selbst weg. Der Cache macht das Kompilieren schnell, soll aber nicht zu hunderten Gigabyte wachsen. Deshalb:

- **Am Ende jeder Sitzung, in der kompiliert wurde:** Größe prüfen (`du -sh target`, unter Windows Rechtsklick → Eigenschaften). Über **20 GB**: `cargo clean`, und im Abschlussbericht sagen, dass es passiert ist.
- **Nach jedem Versionswechsel einer Abhängigkeit** (`cargo update`, Bevy- oder Avian-Upgrade, neue Crate mit großem Baum): einmal `cargo clean`, weil die alten Artefakte sonst liegen bleiben.
- **Kein `--release`**, außer die Aufgabe verlangt es (Performance-Messung); danach `cargo clean --release`.
- **Keine zusätzlichen Target-Verzeichnisse** (`CARGO_TARGET_DIR`) anlegen. Wer in einem eigenen Git-Worktree arbeitet, löscht dessen `target/` zusammen mit dem Worktree.
- **Nie `cargo clean`, während ein anderer Build läuft** (z. B. ein zweiter Agent oder Daniels Editor), und nie fremde Verzeichnisse außerhalb des Repos löschen.

## Den Vault pflegen

- **Konventionen** stehen in `docs/templates/konventionen.md`: Titel „Thema — der Punkt in einem Halbsatz“, Front Matter mit `title:`, Verweise als Obsidian-Links `[[dateiname|Anzeigetext]]`.
- **Eine Quelle pro Tatsache:** nichts abschreiben, was eine andere Seite schon sagt; verlinken. Die Tabelle dazu steht in den Konventionen.
- **Entscheidungen** bekommen keine Prosa, sondern eine ADR nach `docs/templates/adr-nnnn.md` und eine Zeile in `docs/decisions/entscheidungen.md` (dort steht die nächste freie Nummer).
- **Offene Fragen** bekommen eine Seite nach `docs/templates/p-nnn.md` und eine Zeile in `docs/problems/problem-log.md` (dort steht die nächste freie Nummer).
- **Status mitziehen:** Wer etwas baut oder den Planungsstand ändert, aktualisiert `docs/status.md` und das Implementation-Log des Milestones.
- Der Vault ist auf Deutsch; Bezeichner im Code sind englisch.
