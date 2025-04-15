fn generate_binary_sequences(n: usize) -> Vec<String> {
    (0..(1 << n))
        .map(|i| format!("{:0width$b}", i, width = n))
        .collect()
}

fn lcs(a: &str, b: &str) -> usize {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}

fn average_lcs(sequences: &[String]) -> f64 {
    let mut total_lcs = 0;
    let mut count = 0;

    for i in 0..sequences.len() {
        for j in 0..sequences.len() {
            total_lcs += lcs(&sequences[i], &sequences[j]);
            count += 1;
        }
    }
    println!("Total LCS: {}", total_lcs);
    println!("Count: {}", count);
    total_lcs as f64 / count as f64
}

fn main() {
    let sequences = generate_binary_sequences(5);
    let avg_lcs = average_lcs(&sequences);
    println!("Average LCS: {}", avg_lcs);
}
