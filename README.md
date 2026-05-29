# Rust Sinus Konsole

Ein kleines Rust-Projekt, das eine Sinuskurve als ASCII-Grafik in der Konsole ausgibt.

## Ziel

Das Programm berechnet Sinuswerte und zeigt diese mit `*`-Zeichen in der Konsole an.
Zusätzlich wird eine Mittelachse mit `-` dargestellt.

## Beispiel-Ausgabe

```text
          *
        *
------*------------
    *
  *
*
  *
    *
------*------------
        *
          *
```

## Voraussetzungen

Rust und Cargo müssen installiert sein.

Prüfen:

```bash
cargo --version
```

Falls Cargo nicht gefunden wird, Rust installieren:

```text
https://www.rust-lang.org/tools/install
```

## Starten

Repository klonen:

```bash
git clone https://github.com/DEINNAME/rust-sinus-konsole.git
```

In den Ordner wechseln:

```bash
cd rust-sinus-konsole
```

Programm starten:

```bash
cargo run
```

## Code

Der Hauptcode befindet sich hier:

```text
src/main.rs
```

Das Programm verwendet eine Schleife, berechnet für jeden `x`-Wert den Sinus und wandelt den Wert in eine Zeilenposition um. Dort wird dann ein `*` ausgegeben.

## Werte anpassen

Im Code können diese Werte verändert werden:

```rust
let width = 120;
let height = 20;
let angle = x as f64 * 0.2;
```

| Wert     | Bedeutung                |
| -------- | ------------------------ |
| `width`  | Länge der Kurve          |
| `height` | Höhe der Ausgabe         |
| `0.2`    | Streckung der Sinuskurve |

## Was wir gelernt haben

* Rust-Projekt mit Cargo erstellen
* Schleifen verwenden
* Sinus mit `.sin()` berechnen
* Werte auf Konsolenzeilen umrechnen
* einfache ASCII-Grafik ausgeben

## Fazit

Dieses Projekt ist ein einfacher Einstieg in Rust.
Es zeigt, wie man Mathematik, Schleifen und Konsolenausgabe kombiniert.
