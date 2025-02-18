use rand::Rng;
use std::io;

fn main() {
    // Step 1: Read N from user
    println!("Enter a number between 1 and 100:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: usize = input.trim().parse().expect("Please enter a valid number");

    if n < 1 || n > 100 {
        println!("Number out of range! Please enter a number between 1 and 100.");
        return;
    }

    let mut rng = rand::rng();
    let mut trials: Vec<usize> = Vec::new();

    for _ in 0..100 { // Repeat the experiment 100 times
        // Step 2: Generate N random numbers and store them
        let mut numbers: Vec<f32> = (0..n)
            .map(|_| rng.random_range(0.0..1.0))
            .collect();

        let mut count = 0;

        // Step 3: Keep generating numbers until we remove all from the list
        while !numbers.is_empty() {
            count += 1;
            let rand_num = rng.random_range(0.0..1.0);

            // Remove elements that match (allow small precision tolerance)
            numbers.retain(|&x| (x - rand_num).abs() > 1e-5);
        }

        trials.push(count);
    }

    // Step 4: Compute min, max, and avg
    let min = *trials.iter().min().unwrap();
    let max = *trials.iter().max().unwrap();
    let avg: f32 = trials.iter().sum::<usize>() as f32 / trials.len() as f32;

    println!("Minimum iterations: {}", min);
    println!("Maximum iterations: {}", max);
    println!("Average iterations: {:.2}", avg);
}
