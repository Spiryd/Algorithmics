// Cargo.toml:
//
// [dependencies]
// rand = "0.8"

use rand::Rng;
use std::f64::consts::PI;

/// Monte Carlo settings
const N_SAMPLES: u64 = 100_000; // total draws per estimate (must be even, and divisible by S or S^2)
const N_REPEATS: usize = 1_000; // how many independent estimates to compute variance
const S: usize = 10; // number of strata in each dimension

/// Welford’s algorithm for running mean & (population) variance
struct RunningStats {
    count: usize,
    mean: f64,
    m2: f64,
}

impl RunningStats {
    fn new() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }
    fn update(&mut self, x: f64) {
        self.count += 1;
        let delta = x - self.mean;
        self.mean += delta / (self.count as f64);
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;
    }
    /// population variance (divide by count)
    fn var(&self) -> f64 {
        if self.count > 0 {
            self.m2 / (self.count as f64)
        } else {
            0.0
        }
    }
}

/// exact value
const TRUE_VAL: f64 = PI / 4.0;

/// 1D basic
fn mc_1d_basic(n: u64) -> f64 {
    let mut rng = rand::rng();
    let mut sum = 0.0;
    for _ in 0..n {
        let x: f64 = rng.random();
        sum += (1.0 - x * x).sqrt();
    }
    sum / (n as f64)
}

/// 1D stratified
fn mc_1d_stratified(n: u64, strata: usize) -> f64 {
    let mut rng = rand::rng();
    let m = (n as usize) / strata;
    let width = 1.0 / (strata as f64);
    let mut total = 0.0;
    for k in 0..strata {
        let base = (k as f64) * width;
        let mut sum_k = 0.0;
        for _ in 0..m {
            let u: f64 = rng.random_range(0.0..1.0);
            let x = base + u * width;
            sum_k += (1.0 - x * x).sqrt();
        }
        total += sum_k / (m as f64);
    }
    total / (strata as f64)
}

/// 1D antithetic
fn mc_1d_antithetic(n: u64) -> f64 {
    let mut rng = rand::rng();
    let half = n / 2;
    let mut sum = 0.0;
    for _ in 0..half {
        let u: f64 = rng.random();
        sum += (1.0 - u * u).sqrt();
        let v = 1.0 - u;
        sum += (1.0 - v * v).sqrt();
    }
    sum / (n as f64)
}

/// 2D basic: crude accept-reject
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

/// 2D stratified
fn mc_2d_stratified(n: u64, strata: usize) -> f64 {
    let mut rng = rand::rng();
    let cells = strata * strata;
    let m = (n as usize) / cells;
    let width = 1.0 / (strata as f64);
    let mut total = 0.0;
    for i in 0..strata {
        for j in 0..strata {
            let x0 = (i as f64) * width;
            let y0 = (j as f64) * width;
            let mut count_cell = 0usize;
            for _ in 0..m {
                let ux: f64 = rng.random();
                let uy: f64 = rng.random();
                let x = x0 + ux * width;
                let y = y0 + uy * width;
                if y <= (1.0 - x * x).sqrt() {
                    count_cell += 1;
                }
            }
            total += (count_cell as f64) / (m as f64);
        }
    }
    total / (cells as f64)
}

/// 2D antithetic: pair (x,y) with (1-x,1-y)
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

fn main() {
    let mut stats = [
        ("1D basic", RunningStats::new()),
        ("1D stratified", RunningStats::new()),
        ("1D antithetic", RunningStats::new()),
        ("2D basic", RunningStats::new()),
        ("2D stratified", RunningStats::new()),
        ("2D antithetic", RunningStats::new()),
    ];

    for _ in 0..N_REPEATS {
        stats[0].1.update(mc_1d_basic(N_SAMPLES));
        stats[1].1.update(mc_1d_stratified(N_SAMPLES, S));
        stats[2].1.update(mc_1d_antithetic(N_SAMPLES));
        stats[3].1.update(mc_2d_basic(N_SAMPLES));
        stats[4].1.update(mc_2d_stratified(N_SAMPLES, S));
        stats[5].1.update(mc_2d_antithetic(N_SAMPLES));
    }

    println!("True value of ∫₀¹√(1-x²)dx = {:.6}\n", TRUE_VAL);

    // Build a Vec of (name, mean, var), sort by var ascending
    let mut results: Vec<(&str, f64, f64)> = stats
        .iter()
        .map(|(name, st)| (*name, st.mean, st.var()))
        .collect();
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Print ranked list
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
