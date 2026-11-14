---
title: "Intent & Steuerung — MageIntent, InputBuffer, das Flanken-Protokoll"
---
# Intent & Steuerung — MageIntent, InputBuffer, das Flanken-Protokoll

Zwei Datentypen, bewusst getrennt: `MageIntent` ist das, was die Simulation pro Tick konsumiert und was später über das Netz geht. Der `InputBuffer` ist der Akkumulator, in den lokale Intent-Produzenten zwischen den Ticks schreiben. Der Latch-Zähler lebt nur im Puffer, nie im Intent ([[adr-0004-latch-zaehler-fuer-flanken|ADR-0004]]).

```rust
// crates/mage_sim/src/intent.rs (gekürzt)
pub const MAX_SLOTS: usize = 12;

/// Per-Tick-Daten einer Einheit. Keine Entity-, keine Kamerareferenz.
pub struct MageIntent {
    pub move_axis: Vec2,             // Bodenebene, Weltraum, Länge ≤ 1
    pub aim_dir: Vec3,               // Weltraum, normalisiert
    pub held: [bool; MAX_SLOTS],     // in diesem Tick gehalten
    pub pressed: [bool; MAX_SLOTS],  // seit dem letzten Tick mindestens einmal gedrückt
}

/// Was die Einheit in diesem Tick tut, dazu die Flanke aus dem Vortick.
#[derive(Component)]
pub struct IntentForTick { pub intent: MageIntent, pub just_released: [bool; MAX_SLOTS] }

/// Wer die Einheit gerade steuert; bringt IntentForTick als Required Component mit.
#[derive(Component)] #[require(IntentForTick)]
pub enum Controller { Player(PlayerId), Ai(BehaviorId) }

/// Ressource: Eingaben seit dem letzten Tick, nach SimId der gesteuerten Einheit.
pub struct InputBuffer(BTreeMap<SimId, BufferedInput>);
pub struct BufferedInput {
    pub move_axis: Vec2,              // Stichprobe: der letzte Wert gewinnt
    pub aim_dir: Vec3,                // null oder nicht endlich: bisherige Richtung bleibt
    held: [bool; MAX_SLOTS],          // set_held(slot, bool)
    press_count: [u8; MAX_SLOTS],     // press(slot): Latch, saturierend
}
```

**Das Flanken-Protokoll** löst das Problem, dass Frames null oder mehrere fixe Ticks ausführen können:

- Der `HumanController` (Update, Präsentation) liest Aktionen (aus Tastatur und Maus, Controller oder Touch, siehe [[eingabegeraete|Eingabegeräte]]) und schreibt in den `InputBuffer`. `move_axis`, `aim_dir` und `held` sind Stichproben: der letzte Wert gewinnt, nichts wird aufsummiert.
- Druckereignisse werden gezählt: `press_count[i] += 1` (saturierend), je Ereignis, unabhängig von Tickgrenzen.
- Im Set Intent friert `freeze_intents` den Puffer zu einem `MageIntent` ein: `pressed = press_count > 0`, danach wird `press_count` auf 0 gesetzt. `just_released = held_prev && !held` leitet sie aus dem Intent des Vorticks ab. Dabei wird bereinigt: `move_axis` auf Länge 1 geklemmt, nicht endliche Werte werden null; eine ungültige Zielrichtung lässt die bisherige stehen; Slots außerhalb von `MAX_SLOTS` und Einträge unbekannter Einheiten werden ignoriert ([[sicherheit|Sicherheit]]).
- Garantie: Ein Druck geht nie verloren (0 Ticks im Frame: der Latch überlebt bis zum nächsten Tick) und wird nie dupliziert (mehrere Ticks im Frame: nur der erste konsumiert ihn).

Der Intent gehört zur gesteuerten **Einheit**, nicht zum Spieler ([[adr-0031-einheiten-magier-und-kreaturen|ADR-0031]]): Ein Spieler schreibt für die Einheit, an die seine `PlayerId` gebunden ist (auf einem Reittier steuert er so das Reittier), eine KI für ihre Kreaturen. Wer den Intent erzeugt, ist der Simulation egal: Mensch, KI ([[ki-und-rl|KI & RL]]), Add-on ([[add-ons|Add-ons]]), Netzwerk-Replay oder Test. Der Dummy-Gegner ist ein vollwertiger Magier mit einem Intent, der einfach leer bleibt. Auf dem Server kommen die Intents über das Netzwerk und umgehen den Puffer ganz ([[netzwerk|Netzwerk]]). Die Slot-Arrays (`held`, `pressed`) haben `MAX_SLOTS` Einträge; welche Fähigkeit in einem Slot liegt, sagt die Fähigkeitenleiste des Magiers, nicht der Intent ([[faehigkeiten|Fähigkeiten & Belegung]]). Für Flugreittiere bekommt der Intent eine vertikale Achse ([[reittiere|Reittiere]]).
