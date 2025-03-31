fn horner(coeffs: &[f64], x: f64) -> f64 {
    let mut result = 0.0;
    // Iterujemy po współczynnikach w kolejności od najwyższej potęgi do wyrazu wolnego.
    for &a in coeffs.iter().rev() {
        result = a + x * result;
    }
    result
}

fn main() {
    // Przykładowy wielomian: 2 + 3*x + 4*x^2
    let coeffs = vec![2.0, 3.0, 4.0];
    let x = 2.0;
    let value = horner(&coeffs, x);
    println!("Wartość wielomianu dla x = {} wynosi: {}", x, value);
}
