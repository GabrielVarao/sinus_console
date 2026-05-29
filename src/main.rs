// Wir importieren thread und Duration.
// Damit können wir das Programm kurz pausieren lassen,
// damit die Ausgabe in der Konsole etwas angenehmer aussieht.
use std::{thread, time::Duration};

fn main() {
    // width bestimmt, wie viele Sinus-Punkte ausgegeben werden.
    // Je grösser die Zahl, desto länger wird die Kurve.
    let width = 120;

    // height bestimmt, wie hoch unsere ASCII-Grafik ist.
    // Die Sinuskurve wird zwischen 0 und height gezeichnet.
    let height = 20;

    // Diese Schleife läuft von 0 bis width.
    // Für jeden x-Wert berechnen wir einen Sinuswert.
    for x in 0..width {
        // Wir wandeln x in eine Kommazahl um und skalieren sie mit 0.2.
        // Dadurch wird die Sinuskurve schön gestreckt.
        let angle = x as f64 * 0.2;

        // Hier berechnen wir den Sinus.
        // Der Sinus liegt immer zwischen -1.0 und +1.0.
        let sine = angle.sin();

        // Jetzt rechnen wir den Sinuswert auf eine Zeile in der Konsole um.
        //
        // sine + 1.0:
        // Aus dem Bereich -1 bis +1 wird 0 bis 2.
        //
        // * height / 2:
        // Dadurch passt der Wert in unsere Konsolenhöhe.
        let y = ((sine + 1.0) * (height as f64 / 2.0)) as usize;

        // Diese Schleife geht jede Zeile von oben bis unten durch.
        for row in 0..=height {
            // Wenn die aktuelle Zeile der Sinusposition entspricht,
            // zeichnen wir einen Stern.
            if row == y {
                print!("*");
            }
            // In der Mitte zeichnen wir eine Linie als Mittelachse.
            else if row == height / 2 {
                print!("-");
            }
            // Sonst zeichnen wir nur ein Leerzeichen.
            else {
                print!(" ");
            }
        }

        // Nach jeder Spalte gehen wir in eine neue Zeile.
        println!();

        // Kleine Pause, damit die Ausgabe nicht sofort durchrast.
        thread::sleep(Duration::from_millis(30));
    }
}