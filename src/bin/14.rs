use rand::Rng; // Import random number generation traits

/// Length of the bit‐string
const N: usize = 16;

/// Very bad random‐bit generator: X[i] ← random bit
fn random_bits() -> [u8; N] {
    let mut rng = rand::rng(); // Create a random number generator
    let mut x = [0u8; N];      // Initialize an array of N zeros
    for b in &mut x {
        // For each bit in the array
        *b = if rng.random() { 1 } else { 0 }; // Set to 1 or 0 randomly
    }
    x // Return the random bit string
}

/// “Success” = all‐zero string
fn is_success(x: &[u8; N]) -> bool {
    // Return true if all bits are zero, false otherwise
    x.iter().all(|&b| b == 0)
}

/// Conditional‐expectation function
/// c(prefix) = Pr[success | we have already fixed prefix]
fn cond_exp(prefix: &[u8], total_len: usize) -> f64 {
    // If any bit in the prefix is 1, success is impossible
    if prefix.contains(&1) {
        0.0
    } else {
        // Otherwise, probability is 2^-(remaining bits)
        2f64.powi(-((total_len - prefix.len()) as i32))
    }
}

/// Derandomize by fixing bits one by one to maximize c(...)
fn derandomized_bits() -> [u8; N] {
    let mut x = Vec::with_capacity(N); // Will build the bit string one bit at a time
    for _ in 0..N {
        // Try setting the next bit to 0
        x.push(0);
        let c0 = cond_exp(&x, N); // Compute conditional expectation if bit is 0
        *x.last_mut().unwrap() = 1;
        let c1 = cond_exp(&x, N); // Compute conditional expectation if bit is 1
        // Pick the value (0 or 1) that gives higher probability of success
        // If tied, prefer 0
        if c1 > c0 {
            // keep 1 (already set)
        } else {
            *x.last_mut().unwrap() = 0; // set back to 0
        }
    }
    // Convert Vec<u8> to [u8; N]
    let mut arr = [0u8; N];
    arr.copy_from_slice(&x);
    arr
}

fn main() {
    let xr = random_bits();        // Generate a random bit string
    let dr = derandomized_bits();  // Generate a derandomized (all-zero) bit string

    println!("N = {}", N);
    println!("random      = {:?}  success? {}", xr, is_success(&xr));
    println!("derandomized= {:?}  success? {}", dr, is_success(&dr));
    // The derandomized string should always be all zeros and thus always "success"
}
