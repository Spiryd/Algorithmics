use std::collections::{HashMap, HashSet};

/// Buduje prefiksowy automat skończony dla wzorca `p`.
///
/// Automat reprezentowany jest jako wektor map, gdzie:
/// - stany: 0 ..= m (m = długość wzorca), stan m oznacza pełny wzorzec,
/// - każda mapa dla stanu q zawiera przejścia: znak -> kolejny stan.
///
/// Algorytm wzorowany jest na klasycznym podejściu:
///   dfa[0][c] = 0 dla każdego znaku c,
///   dfa[0][P[0]] = 1,
///   dla j = 1..m-1:
///       dla każdego c ∈ alfabet: dfa[j][c] = dfa[longest_prefix_sufix][c]
///       dfa[j][P[j]] = j+1
///       longest_prefix_sufix = dfa[longest_prefix_sufix][P[j]]
fn build_prefix_automaton(p: &str) -> Vec<HashMap<char, usize>> {
    let m = p.len();
    let pattern: Vec<char> = p.chars().collect();

    // Wyznaczamy alfabet – zbiór znaków występujących we wzorcu.
    let alphabet: HashSet<char> = pattern.iter().cloned().collect();

    // Inicjujemy automat: m+1 stanów (od 0 do m)
    let mut dfa: Vec<HashMap<char, usize>> = vec![HashMap::new(); m + 1];

    // Dla stanu 0, ustawiamy przejścia dla wszystkich znaków z alfabetu na 0.
    for &c in &alphabet {
        dfa[0].insert(c, 0);
    }
    // Ustawiamy specyficzne przejście dla pierwszego znaku wzorca.
    dfa[0].insert(pattern[0], 1);

    let mut longest_prefix_sufix = 0; // zmienna pomocnicza określająca najdłuższy prefiks będący sufiksem.
    for j in 1..m {
        // Dla każdego znaku z alfabetu kopiujemy przejścia z automatu w stanie longest_prefix_sufix.
        for &c in &alphabet {
            let next_state = *dfa[longest_prefix_sufix].get(&c).unwrap_or(&0);
            dfa[j].insert(c, next_state);
        }
        // Ustawiamy przejście dla znaku wzorca na aktualnym stanie.
        dfa[j].insert(pattern[j], j + 1);
        // Aktualizujemy longest_prefix_sufix – najdłuższy prefiks będący sufiksem dotychczasowego wzorca.
        longest_prefix_sufix = *dfa[longest_prefix_sufix].get(&pattern[j]).unwrap_or(&0);
    }
    // Opcjonalnie: dla stanu m (pełny wzorzec) również uzupełniamy przejścia.
    for &c in &alphabet {
        let next_state = *dfa[longest_prefix_sufix].get(&c).unwrap_or(&0);
        dfa[m].insert(c, next_state);
    }

    dfa
}

/// Przeszukuje tekst przy użyciu zbudowanego automatu.
/// Funkcja iteruje po kolejnych znakach tekstu, dokonując przejścia między stanami.
/// Jeśli osiągniemy stan równy długości wzorca, oznacza to, że wzorzec został znaleziony.
fn pattern_exists(text: &str, automaton: &Vec<HashMap<char, usize>>, m: usize) -> bool {
    let mut state = 0;
    for c in text.chars() {
        // Jeśli nie mamy zdefiniowanego przejścia dla danego znaku, domyślnie przechodzimy do stanu 0.
        state = *automaton[state].get(&c).unwrap_or(&0);
        if state == m {
            return true;
        }
    }
    false
}

/// Łączy budowę automatu oraz przeszukiwanie tekstu.
/// Zwraca `true`, jeśli wzorzec `pattern` został znaleziony w `text`, w przeciwnym razie `false`.
fn find_pattern(text: &str, pattern: &str) -> bool {
    let automaton = build_prefix_automaton(pattern);
    let m = pattern.len();
    pattern_exists(text, &automaton, m)
}

fn main() {
    let pattern = "abcab";
    let texts = ["xxabcabcabyy", "aaaaaaaaaaabac"];

    for text in texts {
        if find_pattern(text, pattern) {
            println!("Wzorzec '{}' został znaleziony w tekście.", pattern);
        } else {
            println!("Wzorzec '{}' nie został znaleziony w tekście.", pattern);
        }
    }
}
