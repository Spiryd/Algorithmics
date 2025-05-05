use rand::Rng;
use std::f64::consts::PI;

/// Number of random samples to draw
const N_SAMPLES: u64 = 1_000_000;

fn monte_carlo_sin_integral() -> f64 {
    let mut rng = rand::rng();
    let mut sum = 0.0;
    for _ in 0..N_SAMPLES {
        let x = rng.random_range(0.0..PI);
        sum += x.sin();
    }
    // Average value times the interval length
    (sum / N_SAMPLES as f64) * PI
}

fn main() {
    let estimate = monte_carlo_sin_integral();
    println!(
        "Monte Carlo estimate of ∫₀^π sin(x) dx with {} samples = {}",
        N_SAMPLES, estimate
    );
    println!("Exact value = 2.0, error = {}", (estimate - 2.0).abs());
}
