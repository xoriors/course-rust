use rand::Rng;
use std::io;

fn main() {
    println!("Crypto mininig");

    // Read number N from the keyboard
    println!("Enter a number N (between 1 and 100):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input to get N
    let n: u32 = input.trim().parse().expect("Please enter a valide number");

    let mut rng = rand::rng(); // generate random numbers
    let mut results = Vec::new(); // store the results of 100 experiments

    // Run 100 experiments
    for _ in 0..100 {
        let mut collected = vec![false; n as usize]; // Track collected coupons
        let mut collected_count = 0;
        let mut trials = 0;

        // Keep generating until all coupons are collected
        loop {
            let num = rng.random_range(0..n); // Generate random number 0..n-1
            trials += 1;

            // Check if this number hasn't been collected before
            if !collected[num as usize] {
                collected[num as usize] = true;
                collected_count += 1;

                // Exit when all collected
                if collected_count == n {
                    break;
                }
            }
        }

        results.push(trials);
    }

    // Calculate statistics
    let min = results.iter().min().unwrap();
    let max = results.iter().max().unwrap();
    let total: u32 = results.iter().sum();
    let avg = total as f64 / 100.0;

    // Print results
    println!("\nAfter 100 experiments:");
    println!("Minimum trials needed: {}", min);
    println!("Average trials needed: {:.1}", avg);
    println!("Maximum trials needed: {}", max);
}
