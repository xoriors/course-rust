use std::io;
use std::time::Instant;
use cryptoMiningAssignment::random_removal; // Import from lib.rs

fn main() {
    println!("Enter a number between 1 and 100:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Please enter a valid number");

    if n < 1 || n > 100 {
        println!("Number out of range! Please enter a number between 1 and 100.");
        return;
    }

    let mut trials = Vec::new();
    let mut times = Vec::new();

    for _ in 0..100 {
        let start_time = Instant::now();
        let count = random_removal(n);
        let duration = start_time.elapsed().as_secs_f64();

        trials.push(count);
        times.push(duration);
    }

    let min = *trials.iter().min().unwrap();
    let max = *trials.iter().max().unwrap();
    let avg = trials.iter().sum::<usize>() as f32 / trials.len() as f32;

    let min_time = times.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_time = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let avg_time = times.iter().sum::<f64>() / times.len() as f64;

    println!("Minimum iterations: {}", min);
    println!("Maximum iterations: {}", max);
    println!("Average iterations: {:.2}", avg);
    println!("Minimum time taken: {:.6} seconds", min_time);
    println!("Maximum time taken: {:.6} seconds", max_time);
    println!("Average time taken: {:.6} seconds", avg_time);
}
