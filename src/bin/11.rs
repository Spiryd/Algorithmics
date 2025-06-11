use rand::Rng; // Import random number generation traits from the rand crate
use std::f64::consts::PI; // Import the constant value of π

/// Number of random samples to draw for the Monte Carlo estimation
const N_SAMPLES: u64 = 1_000_000;

/// Monte Carlo estimator for ∫₀^π sin(x) dx
///
/// We draw X₁,…,X_N ~ Uniform(0,π) and compute
///   Ĩ_N = π * (1/N) * Σ sin(X_i).
/// This estimator is unbiased:
///   E[Ĩ_N] = ∫₀^π sin(x) dx = 2.
/// Its variance is Var(Ĩ_N) = π² Var(sin(X))/N,
/// with Var(sin(X)) = ½ − 4/π².
/// By the CLT, Ĩ_N ≈ N(2, π²σ²/N) for large N,
/// so you can form approximate confidence intervals:
///   Ĩ_N ± z_{α/2}·π·σ/√N.
fn monte_carlo_sin_integral() -> f64 {
    let mut rng = rand::rng(); // Create a random number generator
    let mut sum = 0.0; // Accumulate the sum of sin(x) values

    // Draw N_SAMPLES random points in [0, π) and sum sin(x) at each point
    for _ in 0..N_SAMPLES {
        let x = rng.random_range(0.0..PI); // Uniform random x in [0, π)
        sum += x.sin(); // Add sin(x) to the running total
    }

    // The average value of sin(x) times the interval length (π) gives the integral estimate
    (sum / N_SAMPLES as f64) * PI
}

fn main() {
    // Compute the Monte Carlo estimate
    let estimate = monte_carlo_sin_integral();

    // Print the estimated value of the integral
    println!(
        "Monte Carlo estimate of ∫₀^π sin(x) dx with {} samples = {}",
        N_SAMPLES, estimate
    );

    // Print the exact value and the absolute error of the estimate
    println!("Exact value = 2.0, error = {}", (estimate - 2.0).abs());
}
