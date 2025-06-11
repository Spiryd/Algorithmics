use rand::Rng; // Import random number generation traits
use rayon::prelude::*;
use std::f64::consts::PI; // Import the constant value of π // Import rayon for parallel iteration

/// Monte Carlo settings
const N_SAMPLES: u64 = 100_000; // Number of samples per estimate (should be even and divisible by S or S^2)
const N_REPEATS: usize = 1_000; // Number of independent estimates to compute variance
const S: usize = 10; // Number of strata in each dimension for stratified sampling

/// Welford’s algorithm for running mean & (population) variance
/// This struct keeps track of the running mean and variance in a numerically stable way.
struct RunningStats {
    count: usize, // Number of samples seen so far
    mean: f64,    // Running mean
    m2: f64,      // Sum of squares of differences from the current mean
}

impl RunningStats {
    /// Create a new RunningStats instance with zeroed fields
    fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }
    /// Update the running statistics with a new sample x
    fn update(&mut self, x: f64) {
        self.count += 1;
        let delta = x - self.mean;
        self.mean += delta / (self.count as f64);
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }
    /// Return the population variance (divide by count, not count-1)
    fn var(&self) -> f64 {
        if self.count > 0 {
            self.m2 / (self.count as f64)
        } else {
            0.0
        }
    }
}

/// The exact value of the integral ∫₀¹√(1-x²)dx, which is π/4
const TRUE_VAL: f64 = PI / 4.0;

/// Monte Carlo: 1D basic sampling
/// Estimate the integral by averaging sqrt(1 - x^2) for x ~ Uniform[0,1]
fn mc_1d_basic(n: u64) -> f64 {
    let mut rng = rand::rng(); // Create a random number generator
    let mut sum = 0.0;
    for _ in 0..n {
        let x: f64 = rng.random(); // Uniform random in [0,1)
        sum += (1.0 - x * x).sqrt();
    }
    sum / (n as f64) // Return the average
}

/// Monte Carlo: 1D stratified sampling
/// Divide [0,1] into `strata` intervals, sample uniformly within each, and average
fn mc_1d_stratified(n: u64, strata: usize) -> f64 {
    let mut rng = rand::rng();
    let m = (n as usize) / strata; // Samples per stratum
    let width = 1.0 / (strata as f64); // Width of each stratum
    let mut total = 0.0;
    for k in 0..strata {
        let base = (k as f64) * width; // Start of the k-th stratum
        let mut sum_k = 0.0;
        for _ in 0..m {
            let u: f64 = rng.random_range(0.0..1.0); // Uniform in [0,1)
            let x = base + u * width; // Map to [base, base+width)
            sum_k += (1.0 - x * x).sqrt();
        }
        total += sum_k / (m as f64); // Average for this stratum
    }
    total / (strata as f64) // Average over all strata
}

/// Monte Carlo: 1D antithetic variates
/// For each sample x, also use 1-x to reduce variance
fn mc_1d_antithetic(n: u64) -> f64 {
    let mut rng = rand::rng();
    let half = n / 2; // Number of pairs
    let mut sum = 0.0;
    for _ in 0..half {
        let u: f64 = rng.random();
        sum += (1.0 - u * u).sqrt();
        let v = 1.0 - u; // Antithetic pair
        sum += (1.0 - v * v).sqrt();
    }
    sum / (n as f64)
}

/// Monte Carlo: 2D basic (crude accept-reject)
/// Estimate the area under the curve by counting points below sqrt(1-x^2)
fn mc_2d_basic(n: u64) -> f64 {
    let mut rng = rand::rng();
    let mut count = 0u64;
    for _ in 0..n {
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        if y <= (1.0 - x * x).sqrt() {
            count += 1;
        }
    }
    (count as f64) / (n as f64)
}

/// Monte Carlo: 2D stratified sampling
/// Divide [0,1]x[0,1] into strata x strata grid, sample uniformly in each cell
fn mc_2d_stratified(n: u64, strata: usize) -> f64 {
    let mut rng = rand::rng();
    let cells = strata * strata; // Total number of grid cells
    let m = (n as usize) / cells; // Samples per cell
    let width = 1.0 / (strata as f64); // Width/height of each cell
    let mut total = 0.0;
    for i in 0..strata {
        for j in 0..strata {
            let x0 = (i as f64) * width; // Cell's x origin
            let y0 = (j as f64) * width; // Cell's y origin
            let mut count_cell = 0usize;
            for _ in 0..m {
                let ux: f64 = rng.random();
                let uy: f64 = rng.random();
                let x = x0 + ux * width; // Map to cell in x
                let y = y0 + uy * width; // Map to cell in y
                if y <= (1.0 - x * x).sqrt() {
                    count_cell += 1;
                }
            }
            total += (count_cell as f64) / (m as f64); // Fraction in this cell
        }
    }
    total / (cells as f64) // Average over all cells
}

/// Monte Carlo: 2D antithetic variates
/// For each (x, y), also use (1-x, 1-y) to reduce variance
fn mc_2d_antithetic(n: u64) -> f64 {
    let mut rng = rand::rng();
    let half = n / 2;
    let mut count = 0u64;
    for _ in 0..half {
        let x: f64 = rng.random();
        let y: f64 = rng.random();
        if y <= (1.0 - x * x).sqrt() {
            count += 1;
        }
        let x2 = 1.0 - x;
        let y2 = 1.0 - y;
        if y2 <= (1.0 - x2 * x2).sqrt() {
            count += 1;
        }
    }
    (count as f64) / (n as f64)
}

/// Main function: runs all estimators, collects statistics, and prints results
fn main() {
    // Array of (method name, Vec<f64>) for each estimator to collect all samples in parallel
    let method_names = [
        "1D basic",
        "1D stratified",
        "1D antithetic",
        "2D basic",
        "2D stratified",
        "2D antithetic",
    ];

    // Use rayon parallel iterator to collect N_REPEATS samples for each method in parallel
    let all_samples: Vec<Vec<f64>> = [
        || {
            (0..N_REPEATS)
                .into_par_iter()
                .map(|_| mc_1d_basic(N_SAMPLES))
                .collect()
        },
        || {
            (0..N_REPEATS)
                .into_par_iter()
                .map(|_| mc_1d_stratified(N_SAMPLES, S))
                .collect()
        },
        || {
            (0..N_REPEATS)
                .into_par_iter()
                .map(|_| mc_1d_antithetic(N_SAMPLES))
                .collect()
        },
        || {
            (0..N_REPEATS)
                .into_par_iter()
                .map(|_| mc_2d_basic(N_SAMPLES))
                .collect()
        },
        || {
            (0..N_REPEATS)
                .into_par_iter()
                .map(|_| mc_2d_stratified(N_SAMPLES, S))
                .collect()
        },
        || {
            (0..N_REPEATS)
                .into_par_iter()
                .map(|_| mc_2d_antithetic(N_SAMPLES))
                .collect()
        },
    ]
    .into_par_iter()
    .map(|f| f())
    .collect();

    // Compute RunningStats for each method from the collected samples
    let stats: Vec<(&str, RunningStats)> = method_names
        .iter()
        .zip(all_samples)
        .map(|(name, samples)| {
            let mut st = RunningStats::new();
            for x in samples {
                st.update(x);
            }
            (*name, st)
        })
        .collect();

    // Print the true value for reference
    println!("True value of ∫₀¹√(1-x²)dx = {:.6}\n", TRUE_VAL);

    // Build a Vec of (name, mean, var), sort by variance ascending (best first)
    let mut results: Vec<(&str, f64, f64)> = stats
        .iter()
        .map(|(name, st)| (*name, st.mean, st.var()))
        .collect();
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Print ranked list of methods by variance
    println!("Ranking of methods by variance (best → worst):");
    for (i, (name, mean, var)) in results.iter().enumerate() {
        println!(
            "{:>2}. {:<15} mean = {:>8.6}, var = {:>10.3e}",
            i + 1,
            name,
            mean,
            var
        );
    }
}
