fn generate_binary_sequences(n: usize) -> Vec<String> {
    (0..(1 << n))
        .map(|i| format!("{:0width$b}", i, width = n))
        .collect()
}

fn main() {
    let sequences = generate_binary_sequences(5);
    for seq in sequences {
        println!("{}", seq);
    }
}
