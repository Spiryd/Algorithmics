/// Funkcja wyznaczająca najdłuższy wspólny podciąg dla trzech ciągów.
/// Zwraca znaleziony LCS jako String.
fn lcs3(s1: &str, s2: &str, s3: &str) -> String {
    // Zamieniamy ciągi na wektory znaków.
    let a: Vec<char> = s1.chars().collect();
    let b: Vec<char> = s2.chars().collect();
    let c: Vec<char> = s3.chars().collect();

    let n = a.len();
    let m = b.len();
    let k = c.len();

    // Tworzymy trójwymiarową tablicę dp o wymiarach (n+1) x (m+1) x (k+1)
    // dp[i][j][l] – długość LCS dla prefiksów a[0..i], b[0..j] oraz c[0..l].
    let mut dp = vec![vec![vec![0; k + 1]; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            for l in 1..=k {
                if a[i - 1] == b[j - 1] && a[i - 1] == c[l - 1] {
                    dp[i][j][l] = dp[i - 1][j - 1][l - 1] + 1;
                } else {
                    dp[i][j][l] = dp[i - 1][j][l]
                        .max(dp[i][j - 1][l])
                        .max(dp[i][j][l - 1]);
                }
            }
        }
    }

    // Rekonstrukcja LCS poprzez backtracking
    let mut i = n;
    let mut j = m;
    let mut l = k;
    let mut lcs_chars = Vec::new();

    while i > 0 && j > 0 && l > 0 {
        if a[i - 1] == b[j - 1] && a[i - 1] == c[l - 1] {
            lcs_chars.push(a[i - 1]);
            i -= 1;
            j -= 1;
            l -= 1;
        } else if dp[i - 1][j][l] >= dp[i][j - 1][l] && dp[i - 1][j][l] >= dp[i][j][l - 1] {
            i -= 1;
        } else if dp[i][j - 1][l] >= dp[i - 1][j][l] && dp[i][j - 1][l] >= dp[i][j][l - 1] {
            j -= 1;
        } else {
            l -= 1;
        }
    }

    lcs_chars.reverse();
    lcs_chars.iter().collect()
}

fn main() {
    let s1 = "AGCAT";
    let s2 = "GAC";
    let s3 = "AGAC";
    
    let result = lcs3(s1, s2, s3);
    println!("Najdłuższy wspólny podciąg dla\n s1 = {}\n s2 = {}\n s3 = {}\n to: {}", s1, s2, s3, result);
}
