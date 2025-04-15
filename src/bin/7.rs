/// Funkcja szukająca wzorca `pattern` w tekście `text` przy użyciu algorytmu Rabina-Karpa.
/// Zwraca wektor indeksów, pod którymi wzorzec występuje w tekście.
fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();
    let mut wynik = Vec::new();

    // Jeśli długość wzorca jest większa niż tekst, zwracamy pusty wynik.
    if m > n {
        return wynik;
    }

    // Parametry haszujące:
    // d – liczba możliwych znaków (np. 256 dla rozszerzonego ASCII)
    // q – liczba pierwsza używana do redukcji kolizji (modulo)
    let d: u64 = 256;
    let q: u64 = 101;

    // Inicjalizacja wartości haszujących:
    let mut hash_pattern = 0u64;
    let mut hash_text = 0u64;
    let mut h = 1u64; // h = d^(m-1) mod q

    // Obliczenie wartości h
    for _ in 0..(m - 1) {
        h = (h * d) % q;
    }

    // Obliczenie początkowych wartości hash dla wzorca i pierwszego okna tekstu
    let pattern_bytes = pattern.as_bytes();
    let text_bytes = text.as_bytes();
    for i in 0..m {
        hash_pattern = (d * hash_pattern + pattern_bytes[i] as u64) % q;
        hash_text = (d * hash_text + text_bytes[i] as u64) % q;
    }

    // Przesuwamy okno przez tekst
    for i in 0..=(n - m) {
        // Jeśli wartości hash się zgadzają, dokonujemy dokładnego porównania znaków
        if hash_pattern == hash_text {
            if &text[i..i + m] == pattern {
                wynik.push(i);
            }
        }

        // Jeśli nie jesteśmy na końcu tekstu, aktualizujemy hash dla kolejnego okna
        if i < n - m {
            // Obliczamy hash dla nowego okna:
            // Odejmujemy wkład pierwszego znaku poprzedniego okna,
            // przesuwamy okno (mnożymy przez d) i dodajemy nowy znak.
            let tmp = (hash_text + q - (text_bytes[i] as u64 * h) % q) % q;
            hash_text = (d * tmp + text_bytes[i + m] as u64) % q;
        }
    }

    wynik
}

fn main() {
    let text = "ala ma kota, a kot ma ale";
    println!("Tekst: {}", text);
    let pattern = "kot";
    println!("Wzorzec: {}", pattern);
    let wystapienia = rabin_karp(text, pattern);

    if wystapienia.is_empty() {
        println!("Wzorzec nie został znaleziony w tekście.");
    } else {
        println!("Wzorzec '{}' znaleziono na pozycjach: {:?}", pattern, wystapienia);
    }
}
