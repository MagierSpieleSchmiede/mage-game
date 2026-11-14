---
title: "Arena & Präsentation — Greybox, Kamera, Render-Target, HUD"
---
# Arena & Präsentation — Greybox, Kamera, Render-Target, HUD

**Arena (Simulation):** Boden, Begrenzungswände und mehrere Hindernisse unterschiedlicher Höhe, alle als statische Kollider ohne jeglichen Zustand. Sie sind Abfragequelle (Shape-Casts, Raycasts für LOS und GroundArea-Zielpunkte) und werden von Rollback-Snapshots nicht berührt, weil sie sich nie ändern.

**Arena (Präsentation):** Dieselbe Geometrie als Greybox-Meshes mit Materialien, die der Simulation egal sind. Simulations-Entities bekommen ihre Visuals, sobald Marker-Components auftauchen (Mage-Marker, Spell-Marker); es gibt keine Szenen-Kopie des Zustands.

**Kamera:** Third-Person-Orbit um den eigenen Magier; Maus dreht Yaw und Pitch. Weil die Kamera seitlich versetzt hinter dem Magier steht, ist ihre Blickrichtung nicht die Zielrichtung: Die Präsentation schickt einen Raycast von der Kamera durch das Fadenkreuz, nimmt den Trefferpunkt (ohne Treffer: einen Punkt in maximaler Reichweite) und schreibt die Richtung vom Magier zu diesem Punkt als `aim_dir` in den Puffer. Die Kamera liest den interpolierten, gerenderten Transform, niemals den rohen Simulationsstand, sonst ruckelt es.

**Rendering:** Die 3D-Szene rendert in ein Render-Target niedriger Auflösung (eine Einstellung, Platzhalter ein Drittel der Fensterhöhe) und wird mit Nearest-Filterung hochskaliert. Der stilisierte Look ist eine Einstellung im Render-Plugin, kein Simulationsanliegen.

**Interpolation:** Die Präsentation hält pro Simulations-Entity eine gerenderte Kopie des Transforms und interpoliert zwischen dem letzten und dem aktuellen fixen Tick anhand des Interpolationsanteils, den Bevy für `FixedUpdate` bereitstellt. Die Simulation selbst bleibt davon unberührt.

**HUD:** Eigenes Leben, Mana, aktive Effekte und der Cooldown-Zustand jedes Slots; Leben der sichtbaren Mitspieler und Gegner in Teamfarben. Das HUD liest ausschließlich die [[beobachtung|Beobachtung]] des eigenen Spielers, fragt nie Eingaben ab und zeigt nur Texte aus der [[lokalisierung|Lokalisierung]].

**Animation:** Reine Präsentation ([[adr-0021-animation-ist-reine-praesentation|ADR-0021]]). Die Simulation entscheidet nach Ticks, wann ein Zauber wirkt; die Animation startet beim Cast, passt ihre Geschwindigkeit an eine eventuelle Wirkzeit an und wird jederzeit abgebrochen oder überblendet, wenn die Simulation etwas Neues tut. Keine Animations-Events lösen Gameplay aus, keine Root Motion bewegt Simulations-Entities. Bis zur Asset-Pipeline genügen einfache prozedurale Bewegungen (Wippen, Ausholen) an der Greybox-Figur.

**Steuerung:** Tastatur und Maus sowie Controller, auf Mobilgeräten Touch; alle über Aktionen mit änderbarer Belegung. Standardbelegung auf [[eingabegeraete|Eingabegeräte]]; die gleiche Tabelle steht im README. Optional später Sprung oder Dash als weitere Aktion.
