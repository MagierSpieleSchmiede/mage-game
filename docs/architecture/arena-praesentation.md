---
title: "Arena & Präsentation — Greybox, Kamera, Render-Target, HUD"
---
# Arena & Präsentation — Greybox, Kamera, Render-Target, HUD

**Arena (Simulation):** Boden, Begrenzungswände und mehrere Hindernisse unterschiedlicher Höhe, alle als statische Kollider ohne jeglichen Zustand. Sie sind Abfragequelle (Shape-Casts, Raycasts für LOS und GroundArea-Zielpunkte) und werden von Rollback-Snapshots nicht berührt, weil sie sich nie ändern.

**Arena (Präsentation):** Dieselbe Geometrie als Greybox-Meshes mit Materialien, die der Simulation egal sind. Simulations-Entities bekommen ihre Visuals, sobald Marker-Components auftauchen (Mage-Marker, Spell-Marker); es gibt keine Szenen-Kopie des Zustands.

**Kamera:** Third-Person-Orbit um den eigenen Magier; Maus dreht Yaw und Pitch. Weil die Kamera seitlich versetzt hinter dem Magier steht, ist ihre Blickrichtung nicht die Zielrichtung: Die Präsentation schickt einen Raycast von der Kamera durch das Fadenkreuz, nimmt den Trefferpunkt (ohne Treffer: einen Punkt in maximaler Reichweite) und schreibt die Richtung vom Magier zu diesem Punkt als `aim_dir` in den Puffer. Die Kamera liest den interpolierten, gerenderten Transform, niemals den rohen Simulationsstand, sonst ruckelt es.

**Rendering:** Die 3D-Szene rendert in ein Render-Target niedriger Auflösung (eine Einstellung, Platzhalter ein Drittel der Fensterhöhe) und wird mit Nearest-Filterung hochskaliert. Der stilisierte Look ist eine Einstellung im Render-Plugin, kein Simulationsanliegen.

**Interpolation:** Die Präsentation hält pro Simulations-Entity eine gerenderte Kopie des Transforms und interpoliert zwischen dem letzten und dem aktuellen fixen Tick anhand des Interpolationsanteils, den Bevy für `FixedUpdate` bereitstellt. Die Simulation selbst bleibt davon unberührt.

**HUD:** Gesundheit und Mana beider Magier sowie der Cooldown-Zustand jedes Slots, als Reaktion auf Health-, Mana- und Cooldown-Components. Das HUD fragt nie Eingaben ab.

**Steuerung (konkret):** WASD Bewegung, Maus Kamera und Ziel, Slots auf Tasten 1/2/3 (Bolt, Homing-Bolt, Feuerball), optional später Sprung oder Dash als vierte Aktion. Die gleiche Tabelle steht im README.
