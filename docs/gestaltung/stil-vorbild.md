---
title: "Stil-Vorbild — nächtliche Pixel-Fantasy in Blau und warmem Licht"
---
# Stil-Vorbild — nächtliche Pixel-Fantasy in Blau und warmem Licht

![[vorbild.jpg|Pixel-Art-Szene: ein Magier in violetter Robe mit Spitzhut, Laterne und leuchtendem Kristallstab steht nachts auf einer Anhöhe neben einer Pflanze; dahinter ein See mit Vollmond-Spiegelung, eine Burg auf einer Insel und ein beleuchtetes Dorf]]

*Herkunft: bitte nachtragen (Werkzeug, Modell, Datum).*

Dieses Bild ist das **Stil-Vorbild** für Mage Game. Es ist 2D-Pixel-Art; unser Spiel ist 3D. Die Brücke ist das Low-Res-Rendering mit harten Pixelkanten ([[adr-0011-low-res-render-target-mit-nearest|ADR-0011]]): Eine einfache 3D-Szene, in niedriger Auflösung gerendert, bekommt von selbst einen pixeligen Look. Übernehmen wollen wir deshalb nicht die Technik, sondern **Farben, Licht und Formensprache**.

## Was wir übernehmen

| Merkmal | Im Vorbild | Im Spiel |
| --- | --- | --- |
| Grundfarben | tiefes Nachtblau, Violett, dunkles Grün | Arena, Boden, Hindernisse und Himmel in kühlen, gedeckten Tönen |
| Akzentlicht | warmes Gelb-Orange (Laterne, Fenster, Mondspiegelung) | Lichtquellen und Feuer: Feuerball, Fackeln, Fenster in der Arena |
| Magie | kaltes, leuchtendes Blau (Kristall am Stab) | Zauber leuchten und heben sich klar vom dunklen Hintergrund ab |
| Magier | klare Silhouette: Spitzhut, lange Robe, Stab; Robe violett mit Goldsaum | erkennbar an der Silhouette; die zwei Spieler unterscheiden sich durch die Robenfarbe |
| Kontrast | dunkle Szene, wenige helle Punkte lenken den Blick | Spieler und Zauber sind die hellsten Dinge im Bild, die Arena tritt zurück |
| Formen | einfache, gut lesbare Formen statt feiner Details | Low-Poly-Modelle, wenige Materialien, kleine oder keine Texturen |

## Was offen ist

- **Tageszeit:** Ist die Arena immer nachts, oder gibt es Varianten? Nachts passt zum Vorbild und lässt Zauber am besten leuchten.
- **Farbe der Spieler:** Welche zwei Robenfarben sind auf dem dunklen Hintergrund gut unterscheidbar, auch für Farbenblinde (z. B. Violett-Gold gegen Türkis-Silber)?
- **Leuchten im Low-Res-Bild:** Ob Glow- und Bloom-Effekte bei niedriger Auflösung gut aussehen, zeigt sich erst ab Milestone 3.

Ergebnisse und neue Entwürfe kommen als eigene Seiten in diesen Ordner, siehe [[einstieg-gestalten|Gestalten]].
