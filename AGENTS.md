# AGENTS.md — hier anfangen

Du arbeitest an **Mage Game**, einem stilisierten 3D-Arena-Duellspiel in Rust mit Bevy 0.20 und Avian3D 0.8. Zwei Magier, freie 3D-Bewegung, Third-Person-Zielen, modulare Zauber (Bolt, Homing-Bolt, Feuerball). Das gesamte Gameplay ist eine deterministische, headless-fähige Tick-Simulation im `FixedUpdate`; die Präsentation ist ein reiner Beobachter. Später kommen KI-Gegner und PvP mit Rollback-Netcode; dafür wird entworfen, aber noch nicht gebaut.

`docs/` ist der **Vault**, das Design-Gedächtnis des Projekts (ein Obsidian-Vault, als Webseite veröffentlicht). Lies ihn, bevor du etwas änderst, und pflege ihn, wenn du es tust.

## Stand

**Implementierung noch nicht begonnen.** `src/main.rs` ist ein Platzhalter. Nächster Schritt laut Vault: Milestone 1 — Walking Skeleton. Fang nicht ungefragt mit Code an; Daniel gibt den Start frei.

## Zuerst lesen

1. `docs/index.md`: die Inhaltskarte des Vaults.
2. `docs/status.md`: was existiert und was fehlt. Wo der Code widerspricht, gewinnt der Code; dann die Seite korrigieren.
3. `docs/architecture/simulationsmodell.md`: die neun nicht verhandelbaren Invarianten und die Tick-Kette.
4. `docs/specifications/spec-001-prototype.md`: Toolchain, Umfang, Anforderungen, Abnahmekriterien.
5. `docs/decisions/`: die ADRs. Was entschieden ist, gilt. Eine angenommene ADR nicht neu verhandeln ohne neue ADR.
6. `docs/problems/problem-log.md`: offene Fragen mit ihrem nächsten Schritt.
7. `docs/milestones/roadmap.md` und `docs/testing/teststrategie.md`: was in welchem Milestone dran ist und wie es getestet wird.
8. `docs/templates/konventionen.md`: wie Seiten, ADRs und P-Einträge geschrieben und abgelegt werden.

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
- **Balance-Konstanten** gehören in die zentrale Balance-Ressource, nie als magische Zahlen in Systeme.
- **Umfang:** Keine Abstraktion, die nicht der „Entwirf für“-Liste dient oder mindestens zweimal verwendet wird. Keine TODO-Stubs für Kernfunktionalität. Netzwerk, KI, Menüs, Audio, Asset-Pipelines, Spielstände, Animation sind außerhalb des Prototyps.
- **`Cargo.lock` wird eingecheckt.** Neue Abhängigkeiten nur mit Begründung (ADR, wenn es eine Architekturfrage ist).
- **Nie direkt auf `master` arbeiten.** Eigener Branch, Pull Request; gemergt wird, wenn Daniel es sagt. Pushen nur nach Freigabe.
- **Nie committen:** `target/`, `site/.quartz/`, `site/public/`, `.env`, `.secrets`.

## Den Vault pflegen

- **Konventionen** stehen in `docs/templates/konventionen.md`: Titel „Thema — der Punkt in einem Halbsatz“, Front Matter mit `title:`, Verweise als Obsidian-Links `[[dateiname|Anzeigetext]]`.
- **Entscheidungen** bekommen keine Prosa, sondern eine ADR nach `docs/templates/adr-nnnn.md`. Nächste freie Nummer steht in den Konventionen (derzeit ADR-0015).
- **Offene Fragen** bekommen eine Seite nach `docs/templates/p-nnn.md` und eine Zeile in `docs/problems/problem-log.md` (derzeit P-006).
- **Status mitziehen:** Wer etwas baut, aktualisiert `docs/status.md` und das Implementation-Log des Milestones (`docs/milestones/mN-implementation-log.md`).
- Nach neuen ADRs oder P-Einträgen die nächste freie Nummer in den Konventionen bzw. im Problem-Log hochzählen.
- Der Vault ist auf Deutsch; Bezeichner im Code sind englisch.
