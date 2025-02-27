use std::time::Instant;

fn complex_calculation(n: u64) -> u64 {
    let mut total: u64 = 0;
    for i in 0..n {
        total = total.wrapping_add(i * i);
        if i % 2 == 0 {
            total = total.wrapping_add(i);
        } else {
            total = total.wrapping_sub(i);
        }
    }
    total
}

fn main() {
    let n: u64 = 100_000_000;

    let start = Instant::now();
    let result = complex_calculation(n);
    let duration = start.elapsed();

    println!("Rust Results:");
    println!("Result: {}", result);
    println!("Time taken: {:.4} seconds", duration.as_secs_f64());
}
