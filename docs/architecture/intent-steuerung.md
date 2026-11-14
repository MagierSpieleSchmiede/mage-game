---
title: "Intent & Steuerung — MageIntent, InputBuffer, das Flanken-Protokoll"
---
# Intent & Steuerung — MageIntent, InputBuffer, das Flanken-Protokoll

Zwei Datentypen, bewusst getrennt: `MageIntent` ist das, was die Simulation pro Tick konsumiert und was später über das Netz geht. Der `InputBuffer` ist der Akkumulator, in den lokale Intent-Produzenten zwischen den Ticks schreiben. Der Latch-Zähler lebt nur im Puffer, nie im Intent ([[adr-0004-latch-zaehler-fuer-flanken|ADR-0004]]).

```rust
/// Per-Tick-Daten. Serde-serialisierbar. Keine Entity-, keine Kamerareferenz.
pub struct MageIntent {
    /// Normalisierte Bewegungsachse in der Bodenebene, bereits weltseitig gedreht.
    pub move_axis: Vec2,
    /// Zielrichtung im Weltraum, normalisiert.
    pub aim_dir: Vec3,
    /// Slot 1 bis 3: Taste in diesem Tick gehalten.
    pub held: [bool; 3],
    /// Slot 1 bis 3: Taste seit dem letzten Tick mindestens einmal gedrückt.
    pub pressed: [bool; 3],
}

/// Eintrag im InputBuffer (Ressource, nach der SimId der gesteuerten Einheit).
/// Wird in Update beschrieben und im Set Intent konsumiert.
pub struct BufferedInput {
    pub move_axis: Vec2,
    pub aim_dir: Vec3,
    pub held: [bool; 3],
    /// Latch: Druckereignisse seit dem letzten konsumierten Tick (saturierend).
    pub press_count: [u8; 3],
}
```

**Das Flanken-Protokoll** löst das Problem, dass Frames null oder mehrere fixe Ticks ausführen können:

- Der `HumanController` (Update, Präsentation) liest Aktionen (aus Tastatur und Maus, Controller oder Touch, siehe [[eingabegeraete|Eingabegeräte]]) und schreibt in den `InputBuffer`. `move_axis`, `aim_dir` und `held` sind Stichproben: der letzte Wert gewinnt, nichts wird aufsummiert.
- Druckereignisse werden gezählt: `press_count[i] += 1` (saturierend), je Ereignis, unabhängig von Tickgrenzen.
- Im Set Intent friert die Simulation den Puffer zu einem `MageIntent` ein: `pressed = press_count > 0`, danach wird `press_count` auf 0 gesetzt. `just_released = held_prev && !held` leitet sie aus dem Intent des Vorticks ab.
- Garantie: Ein Druck geht nie verloren (0 Ticks im Frame: der Latch überlebt bis zum nächsten Tick) und wird nie dupliziert (mehrere Ticks im Frame: nur der erste konsumiert ihn).

Der Intent gehört zur gesteuerten **Einheit**, nicht zum Spieler ([[adr-0031-einheiten-magier-und-kreaturen|ADR-0031]]): Ein Spieler schreibt für die Einheit, an die seine `PlayerId` gebunden ist (auf einem Reittier steuert er so das Reittier), eine KI für ihre Kreaturen. Wer den Intent erzeugt, ist der Simulation egal: Mensch, KI ([[ki-und-rl|KI & RL]]), Add-on ([[add-ons|Add-ons]]), Netzwerk-Replay oder Test. Der Dummy-Gegner ist ein vollwertiger Magier mit einem Intent, der einfach leer bleibt. Auf dem Server kommen die Intents über das Netzwerk und umgehen den Puffer ganz ([[netzwerk|Netzwerk]]). Die Slot-Arrays (`held`, `pressed`) haben `MAX_SLOTS` Einträge; welche Fähigkeit in einem Slot liegt, sagt die Fähigkeitenleiste des Magiers, nicht der Intent ([[faehigkeiten|Fähigkeiten & Belegung]]). Für Flugreittiere bekommt der Intent eine vertikale Achse ([[reittiere|Reittiere]]).
