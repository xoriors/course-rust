use std::io;
use rand::Rng;

// Normal assignment
fn crypto_mining(size: usize) {
    let error_margin : f32 = 1e-6;
    let mut iter_gen: [u32; 10] = [0; 10];

    for i in 0..10 {
        let mut numbers : Vec<f32> = vec![0.0; size];

        // Generates the random numbers to be guessed
        for i in 0..size {
            numbers[i] = rand::thread_rng().gen_range(0.0..=1.0);
        }

        // Generates rand numbers and keeps those that don't match
        while !numbers.is_empty() {
            let guess = rand::thread_rng().gen_range(0.0..=1.0);

            numbers.retain(|&x| (x - guess).abs() > error_margin);

            iter_gen[i] += 1;
        }
    }

    let sum : u32 = iter_gen.iter().sum();
    let min = iter_gen.iter().min().unwrap();
    let avg = sum / 10;
    let max = iter_gen.iter().max().unwrap();

    println!("STATS NUMBERS GENERATED:\nMin: {}\nAvg: {}\nMax: {}", min, avg, max);
}

// Advanced assignment
fn weighted_crypto_mining(size: usize) {

}

fn main() {
    println!("Select how many numbers to generate: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n : usize = n.trim().parse().expect("Expected an unsigned integer!");

    if n == 0 || n > 100 {
        println!("Expected a number lower than 100!");
        return;
    }

    crypto_mining(n);

}
