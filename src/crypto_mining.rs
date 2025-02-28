use rand::Rng;
use std::io;

pub fn main() {
   
    println!("Enter a number N between 1 and 100 ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

   
    if n < 1 || n > 100 {
        println!("N must be between 1 and 100");
        return;
    }

   
    let mut results = Vec::new();
    for _ in 0..100 {
        let count = repeat_experiment(n);
        results.push(count);
    }

  
    let min = results.iter().min().unwrap();
    let max = results.iter().max().unwrap();
    let avg = results.iter().sum::<u32>() as f32 / results.len() as f32;
   
    println!("\nResults for N = {}:", n);
    println!("Min attempts: {}", min);
    println!("Average attempts: {:.2}", avg);
    println!("Max attempts: {}", max);
}


fn repeat_experiment(n: usize) -> u32 {
    let mut rng = rand::thread_rng();
    
    let mut targets: Vec<f32> = (0..n).map(|_| rng.gen_range(0.0..1.0)).collect();
    let mut count = 0;

   
    while !targets.is_empty() {
        let random_num = rng.gen_range(0.0..1.0);
      
        if let Some(pos) = targets.iter().position(|&x| (x - random_num).abs() < 0.01) {
            targets.remove(pos);
        }
        count += 1;
    }
    count
}
