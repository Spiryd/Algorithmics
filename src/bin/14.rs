// Cargo.toml
// [dependencies]
// rand = "0.8"

use rand::Rng;

/// Length of the bit‐string
const N: usize = 16;

/// Very bad random‐bit generator: X[i] ← random bit
fn random_bits() -> [u8; N] {
    let mut rng = rand::rng();
    let mut x = [0u8; N];
    for b in &mut x {
        *b = if rng.random() { 1 } else { 0 };
    }
    x
}

/// “Success” = all‐zero string
fn is_success(x: &[u8; N]) -> bool {
    x.iter().all(|&b| b == 0)
}

/// Conditional‐expectation function
/// c(prefix) = Pr[success | we have already fixed prefix]
fn cond_exp(prefix: &[u8], total_len: usize) -> f64 {
    if prefix.iter().any(|&b| b == 1) {
        0.0
    } else {
        2f64.powi(-((total_len - prefix.len()) as i32))
    }
}

/// Derandomize by fixing bits one by one to maximize c(...)
fn derandomized_bits() -> [u8; N] {
    let mut x = Vec::with_capacity(N);
    for _ in 0..N {
        // try setting next bit = 0 or = 1
        x.push(0);
        let c0 = cond_exp(&x, N);
        *x.last_mut().unwrap() = 1;
        let c1 = cond_exp(&x, N);
        // pick the better (ties go to 0)
        if c1 > c0 {
            // keep 1
        } else {
            *x.last_mut().unwrap() = 0;
        }
    }
    // convert Vec<u8> to [u8; N]
    let mut arr = [0u8; N];
    arr.copy_from_slice(&x);
    arr
}

fn main() {
    let xr = random_bits();
    let dr = derandomized_bits();

    println!("N = {}", N);
    println!("random      = {:?}  success? {}", xr, is_success(&xr));
    println!("derandomized= {:?}  success? {}", dr, is_success(&dr));
}
