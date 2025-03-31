#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Nucleotide {
    A,
    T,
    G,
    C,
}

// Implementacja konwersji z char na Nucleotide.
impl TryFrom<char> for Nucleotide {
    type Error = String;
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            'A' => Ok(Nucleotide::A),
            'T' => Ok(Nucleotide::T),
            'G' => Ok(Nucleotide::G),
            'C' => Ok(Nucleotide::C),
            _   => Err(format!("Niepoprawny symbol: {}", ch)),
        }
    }
}

// Funkcja konwertująca ciąg &str na wektor Nucleotide.
fn parse_sequence(seq: &str) -> Result<Vec<Nucleotide>, String> {
    seq.chars().map(Nucleotide::try_from).collect()
}

// Funkcje pomocnicze do rozpoznawania kodonów.
fn is_start_codon(slice: &[Nucleotide]) -> bool {
    slice == [Nucleotide::A, Nucleotide::T, Nucleotide::G]
}

fn is_stop_codon(slice: &[Nucleotide]) -> bool {
    slice == [Nucleotide::T, Nucleotide::A, Nucleotide::A] ||
    slice == [Nucleotide::T, Nucleotide::A, Nucleotide::G] ||
    slice == [Nucleotide::T, Nucleotide::G, Nucleotide::A]
}

// Wewnątrz u nie może wystąpić żaden z zakazanych kodonów, czyli:
// "ATG", "TAA", "TAG", "TGA"
fn is_forbidden_codon(slice: &[Nucleotide]) -> bool {
    is_start_codon(slice) || is_stop_codon(slice)
}

// Funkcja przeszukująca ciąg w poszukiwaniu fragmentów genu o postaci:
// "ATG" + u + F, gdzie F to kodon stop, |u| >= 30, a w u nie występują
// żadne kodony z { "ATG", "TAA", "TAG", "TGA" }.
fn find_genes(seq: &[Nucleotide]) -> Vec<(usize, usize)> {
    let n = seq.len();
    // Tablica, która dla każdej pozycji i (gdzie może zaczynać się kodon) wskaże
    // pierwsze wystąpienie zakazanego kodonu od pozycji i.
    let mut next_forbidden = vec![n; n];
    
    // Przetwarzamy od końca – tylko dla pozycji, gdzie mamy pełny kodon (do n-3).
    for i in (0..=n.saturating_sub(3)).rev() {
        if is_forbidden_codon(&seq[i..i+3]) {
            next_forbidden[i] = i;
        } else if i + 1 < n {
            next_forbidden[i] = next_forbidden[i+1];
        }
    }
    
    let mut genes = Vec::new();
    // Przeszukujemy ciąg w poszukiwaniu startowego kodonu "ATG".
    for i in 0..=n.saturating_sub(3) {
        if is_start_codon(&seq[i..i+3]) {
            let u_start = i + 3;
            if u_start >= n { continue; }
            // j – pozycja pierwszego wystąpienia zakazanego kodonu po u_start.
            let j = next_forbidden[u_start];
            // Sprawdzamy, czy znaleziono zakazany kodon, u jest wystarczająco długie
            // oraz czy zakazany kodon jest jednym ze stop kodonów.
            if j < n && j >= u_start + 30 && j + 3 <= n {
                if is_stop_codon(&seq[j..j+3]) {
                    // Dodajemy wynik jako parę (indeks początkowy, indeks końcowy),
                    // gdzie fragment to seq[i..j+3].
                    genes.push((i, j + 3));
                }
            }
        }
    }
    genes
}

// Funkcja pomocnicza do konwersji fragmentu wektora Nucleotide na String.
fn nucleotides_to_string(slice: &[Nucleotide]) -> String {
    slice.iter().map(|n| match n {
        Nucleotide::A => 'A',
        Nucleotide::T => 'T',
        Nucleotide::G => 'G',
        Nucleotide::C => 'C',
    }).collect()
}

fn main() {
    // Przykładowy ciąg – można go modyfikować wg potrzeb.
    let sequence_str = "TTTATGCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCTAAATGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAATAG";
    // Konwertujemy ciąg na wektor Nucleotide.
    let sequence = parse_sequence(sequence_str).expect("Błąd podczas parsowania sekwencji");
    
    // Znajdujemy geny wg zadanych warunków.
    let genes = find_genes(&sequence);
    
    if genes.is_empty() {
        println!("Nie znaleziono żadnych kandydatów.");
    } else {
        for (start, end) in genes {
            let gene_str = nucleotides_to_string(&sequence[start..end]);
            println!("Znaleziono gen: {}", gene_str);
        }
    }
}
