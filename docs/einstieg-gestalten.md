---
title: "Gestalten — mit Blender und ArtCraft am Look des Spiels arbeiten"
---
# Gestalten — mit Blender und ArtCraft am Look des Spiels arbeiten

Diese Seite ist für alle, die das Spiel **gestalten** wollen: Magier, Zauber-Effekte, Arena, Farben, Stimmung. Programmieren musst du dafür nicht. Du brauchst GitHub, Git und Obsidian aus [[einstieg-windows|Einstieg für Mitwirkende (Windows)]]; Rust und die Build Tools nur, wenn du deine Modelle selbst im Spiel ansehen willst.

## 1. Was gerade gebraucht wird

Der Prototyp läuft bewusst mit **Greybox**: graue Blöcke statt fertiger Grafik, damit zuerst das Spielgefühl stimmt. Eine Asset-Pipeline (fertige Modelle und Texturen im Spiel) gehört noch nicht zum Prototyp. Deshalb ist Gestaltung im Moment vor allem **Vorbereitung**:

- **Look entwickeln:** Moodboards, Farbpaletten, Licht und Stimmung der Arena.
- **Magier entwerfen:** Silhouetten, Kleidung, Farben, so dass man die beiden Spieler auf einen Blick unterscheidet.
- **Zauber sichtbar machen:** Wie sehen Bolt, Homing-Bolt und Feuerball aus? Wie sieht man, dass ein Feuerball gleich einschlägt (die Fuse)? Wie wirkt Verlangsamung?
- **Arena entwerfen:** Grundriss, Hindernisse unterschiedlicher Höhe, Deckung.
- **Erste Modelle:** einfache Low-Poly-Modelle zum Ausprobieren in Blender.

## 2. Der Stil: Low-Res mit harten Pixeln

Unser Stil-Vorbild ist eine nächtliche Pixel-Fantasy-Szene: Nachtblau und Violett, warmes Laternenlicht, leuchtende Magie. Was wir daraus übernehmen, steht auf [[stil-vorbild|Stil-Vorbild]].

![[vorbild.jpg|Stil-Vorbild: Magier mit leuchtendem Stab bei Nacht, Vollmond über einem See]]

Das Spiel rendert die 3D-Szene in **niedriger Auflösung** (etwa ein Drittel der Fensterhöhe) und skaliert sie mit harten Pixelkanten hoch ([[adr-0011-low-res-render-target-mit-nearest|ADR-0011]]). Daraus folgt für die Gestaltung:

- **Klare Silhouetten** schlagen feine Details; was kleiner als ein paar Pixel ist, verschwindet.
- **Wenige, kräftige Farben** und klare Hell-Dunkel-Kontraste.
- **Low-Poly:** Ein Magier braucht keine tausende Polygone; Formen lesen sich über die Umrisse.
- **Kleine Texturen** (64 bis 256 Pixel) oder ganz ohne Textur, nur mit Farbflächen.

## 3. Blender — Modelle bauen

[Blender](https://www.blender.org) ist kostenlos und das Standardwerkzeug für 3D-Modelle. Installieren:

```powershell
winget install --id BlenderFoundation.Blender -e
```

Für den Einstieg reichen die offiziellen Tutorials auf blender.org und das Modellieren mit einfachen Grundformen (Würfel, Zylinder, Extrudieren).

**Regeln, damit ein Modell später ins Spiel passt:**

| Regel | Warum |
| --- | --- |
| 1 Blender-Einheit = 1 Meter; ein Magier ist etwa 1,8 m groß | Das Spiel rechnet in Metern; Kollisionen und Reichweiten hängen daran |
| Ursprung (Origin) an den Füßen; die Figur schaut in Blenders Vorderansicht (Numpad 1) zu dir | Das Spiel stellt Figuren an ihrem Ursprung auf den Boden; die Drehung ins Spiel übernimmt der Code |
| Vor dem Export: *Object → Apply → All Transforms* | Sonst kommen Modelle verdreht oder falsch skaliert an |
| Export als **glTF 2.0 (.glb)** mit *+Y Up* (Standard des Exporters) | Bevy, unsere Engine, lädt glTF direkt |
| Wenige Materialien, sprechende Namen (`robe`, `hut`, `stab`) | Leichter zu ersetzen und zu färben |
| Die `.blend`-Quelldatei immer mit abgeben | Nur so kann jemand anders das Modell später ändern |

## 4. ArtCraft — Ideen schnell sichtbar machen

[ArtCraft](https://getartcraft.com) ist eine kostenlose, quelloffene Desktop-App für KI-gestützte Bilder und Szenen (Windows, macOS, Linux; Anmeldung mit kostenlosem Konto). Sie kombiniert ein 3D-Viewport zum Aufbauen von Szenen (Figuren posen, Kamera setzen, Objekte zusammenstellen) mit Bild-, Video- und 3D-Modellen verschiedener Anbieter.

Wofür wir es nutzen:

- **Konzeptbilder und Moodboards:** Eine grob gestellte Szene (Arena, Magier zweier Teams, Kamerawinkel) in verschiedene Stile rendern lassen, um den Look zu finden.
- **Varianten:** Dieselbe Magier-Pose in fünf Farbpaletten, derselbe Feuerball in drei Größen.
- **Bild zu 3D:** Aus einem Konzeptbild ein grobes 3D-Modell erzeugen. Das ist ein **Startpunkt**, kein fertiges Asset: Solche Modelle haben zu viele Polygone und unsaubere Geometrie. Sie gehen immer durch Blender (vereinfachen, Maßstab, Ursprung, Materialien) nach den Regeln oben.

**Wichtig bei KI-Bildern und -Modellen:** Das Projekt steht unter der MIT-Lizenz. Bei KI-Erzeugnissen hängt die Nutzbarkeit von den Bedingungen des jeweiligen Modells ab. Notiere deshalb bei jedem Bild und Modell, **mit welchem Werkzeug und welchem Modell** es entstanden ist. Keine Bilder, Figuren oder Stile bekannter Spiele, Filme oder Künstler nachbauen lassen.

## 5. Wohin mit den Ergebnissen?

Ideen und Entwürfe landen im Vault, damit alle sie sehen, auch auf der Webseite:

- **Eine Seite pro Thema** unter `docs/gestaltung/`, z. B. `docs/gestaltung/magier.md` oder `docs/gestaltung/feuerball.md`. Aufbau wie jede Vault-Seite ([[konventionen|Konventionen]]): was vorgeschlagen wird, warum, was offen ist.
- **Bilder** daneben unter `docs/gestaltung/bilder/`, als PNG oder JPG, nicht größer als nötig (etwa 1–2 MB pro Bild). In Obsidian einfach in die Seite ziehen.
- **Herkunft** unter jedes Bild: Werkzeug, Modell, Datum.

Wo **Modelldateien** (`.blend`, `.glb`) im Repo liegen und ob große Dateien mit Git LFS gespeichert werden, ist noch nicht entschieden ([[p-006-asset-ablage-und-grosse-binaerdateien|P-006]]). Bis dahin: Modelle nicht ins Repo committen, sondern in der Gruppe teilen und auf der Gestaltungsseite beschreiben.

## 6. Arbeitsablauf

Derselbe wie für alle, siehe [[einstieg-windows|Einstieg für Mitwirkende (Windows)]]: pullen, Seite oder Bilder in Obsidian ergänzen, in GitHub Desktop committen, pushen. Größere Ideen vorher kurz im Chat ansprechen.
