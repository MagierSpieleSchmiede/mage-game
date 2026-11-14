---
title: "P-006 — Asset-Ablage und große Binärdateien"
---
# P-006 — Asset-Ablage und große Binärdateien

Gestaltende arbeiten mit Blender-Dateien (`.blend`), exportierten Modellen (`.glb`) und Texturen. Diese Dateien sind binär und können groß werden; Git speichert jede Version vollständig, das Repo wächst damit für alle Mitwirkenden. Offen ist: wo Quell- und Exportdateien liegen (Vorschlag: Quellen unter `art/`, exportierte Spieldateien unter `assets/`, wo Bevy sie standardmäßig sucht), ab welcher Größe Git LFS nötig ist, und wie Herkunft und Lizenz eines Assets (besonders bei KI-Erzeugnissen) festgehalten werden. Bis zur Entscheidung werden Modelldateien nicht committet; Bilder im Vault bleiben klein.

Nächster Schritt: vor dem ersten echten Asset (spätestens nach M3, wenn die Greybox ersetzt wird) eine ADR schreiben: Ordnerstruktur, Git LFS ja/nein mit `.gitattributes`, Herkunftsangabe pro Asset.
