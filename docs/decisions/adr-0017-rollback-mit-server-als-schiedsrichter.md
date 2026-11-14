---
title: "ADR-0017 — Rollback-Netcode, wo immer möglich; der Server ist Schiedsrichter"
---
# ADR-0017 — Rollback-Netcode, wo immer möglich; der Server ist Schiedsrichter

**Status:** angenommen (2026-11-14, Richtung von Daniel: Rollback behalten, wo es geht; im Zweifel ist der Server maßgebend)

**Kontext:** PvP war als Rollback-Netcode geplant: Jeder Client simuliert alles mit den Eingaben aller Spieler und spult zurück, wenn eine Eingabe anders ausfiel als vorhergesagt. Das fühlt sich ohne Verzögerung an und braucht wenig Bandbreite. Neue Anforderungen setzen Grenzen: Bei Unsichtbarkeit darf ein Client die Eingaben und den Zustand unsichtbarer Gegner nicht kennen, sonst ist ein Wallhack trivial ([[adr-0018-sichtbarkeit-als-simulationszustand|ADR-0018]]). Mit vielen Spielern verraten Peer-to-Peer-Verbindungen IP-Adressen und skalieren schlecht. Und plattformübergreifend ist Bit-Gleichheit nicht garantiert ([[p-003-cross-plattform-determinismus-fuer-echtes-pvp|P-003]]).

**Entscheidung:** **Rollback-Netcode mit einem Server als Schiedsrichter.**

1. Alle Clients verbinden sich mit einem Server, nie untereinander. Der Server führt die maßgebliche Simulation.
2. Der Server leitet jedem Client die **Intents der Spieler weiter, die dessen Team sieht**. Damit simuliert jeder Client alles, was er sehen darf, selbst: eigene Eingaben sofort, fremde Eingaben vorhergesagt (letzte Eingabe gilt weiter); kommt die echte Eingabe an und weicht ab, spult der Client zurück und simuliert neu. Das ist klassischer Rollback.
3. Wo Rollback nicht reicht, schickt der Server **Zustand**: wenn eine Entity für ein Team neu sichtbar wird, wenn unsichtbare Gegner auf Sichtbares wirken (ein Treffer aus dem Verborgenen, ein Zusammenstoß), und regelmäßig als Prüfstand.
4. **Im Zweifel gilt der Server.** Weicht der Zustand eines Clients für einen bestätigten Tick vom Server ab, spult der Client zu diesem Tick zurück, übernimmt den Server-Stand und simuliert seine Eingaben neu.
5. Eingaben und Zustand unsichtbarer Gegner verlassen den Server nicht in Richtung des Teams, das sie nicht sieht.

**Verworfene Alternativen:** Reiner Peer-to-Peer-Rollback (jeder bekommt alle Eingaben): verworfen wegen Wallhack bei Unsichtbarkeit und IP-Leaks. Reine Server-Autorität mit Vorhersage nur des eigenen Magiers und Interpolation aller anderen: verworfen, weil sie fremde Magier in der Vergangenheit zeigt und das Rollback-Gefühl ohne Not aufgibt, wo die Information ohnehin sichtbar ist.

**Konsequenzen:** Die Simulation muss auf dem Client einen Teil der Welt rechnen können (nur Sichtbares) und Zustand vom Server mitten in den Verlauf einfügen können. Rollback kostet Rechenzeit: n Ticks Rückspulen heißt n Ticks neu rechnen, das begrenzt das Rollback-Fenster auf alten Rechnern ([[benchmarks|Benchmarks]]). Determinismus bleibt zentral: Je genauer Clients und Server übereinstimmen, desto seltener korrigiert der Server. Unsichtbarkeit kostet an genau den Stellen Rollback-Qualität, wo Information fehlt. Details: [[netzwerk|Netzwerk]], Sicherheit: [[sicherheit|Sicherheit]].
