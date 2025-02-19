use rand::{distributions::{Uniform, WeightedIndex}, prelude::Distribution, Rng};
use std::io;

/// Read a number from stdin
/// `prompt` The prompt to display to the user
/// `read_err` The error to display if the reading operation failed
/// `lower_limit` The lower limit of the number
/// `upper_limit` The upper limit of the number
fn read_number(prompt: &str, read_err: &str, lower_limit: u16, upper_limit: u16) -> u16 {
    let n: u16 = loop {
        let mut n = String::new();
        println!("{prompt}");
        io::stdin().read_line(&mut n).expect(read_err);

        let n: u16 = match n.trim().parse() {
            Ok(nr) => nr,
            Err(_) => continue,
        };

        if lower_limit <= n && n <= upper_limit {
            break n;
        } else {
            println!("{n} is not between {lower_limit} and {upper_limit}!");
        }
    };

    n
}

/// The regular/simple part of the homework
/// `n` The number of random numbers to sample
fn simple(n: u16) -> () {
    // keeps track of how many iterations have happened in each attempt
    let mut iter_index: [u32; 10] = [0; 10];
    for i in 0..10 {
        let mut rand_nums: Vec<f32> = Uniform::new(0.0, 1.0)
                                        .sample_iter(rand::thread_rng())
                                        .take(n.into())
                                        .collect();

        while !rand_nums.is_empty() {
            iter_index[i] += 1;
            let num = rand::thread_rng().gen_range(0.0..1.0);
            //check if the random number is in the random numbers vector
            let pos = rand_nums.iter().position(|&x| x == num);

            //if it is, remove it
            //otherwise, continue
            match pos {
                Some(pos) => {
                    let _ = rand_nums.swap_remove(pos);
                },
                None => continue,
            }
        }

    }

    //calculate the min, avg and max iterations it took for each attempt
    let sum: u32 = iter_index.iter().sum();
    let avg = (sum as f32) / (iter_index.len() as f32);
    let min = iter_index.iter().min().unwrap();
    let max = iter_index.iter().max().unwrap();
    println!("Min: {:?}; Avg: {avg}; Max: {:?}", min, max);
}

/// The "adventurous" part of the homework
/// `n` The number of random numbers to sample
fn advanced(n: u16) {
    // keeps track of how many iterations have happened in each attempt
    let mut iter_index: [u32; 10] = [0; 10];
    for i in 0..10 {
        let mut rand_nums: Vec<u64> = Uniform::new(1, 1_000_000)
                                        .sample_iter(rand::thread_rng())
                                        .take(n.into())
                                        .collect();
        //each index represents the interval [index * 100, (index + 1) * 100]
        //for example, weights[0] represents [1, 100], weights[2] [101, 200] etc
        let mut weights: [u64; 10_000] = [0; 10_000];
        for num in &rand_nums {
            //subtracting one to add the number to the proper interval
            //for example, adding 100 to the [1, 100] interval (index 0)
            //instead of [101, 200] (index 1)
            let interval_index = (num - 1) / 100;
            weights[interval_index as usize] += 1;
        }

        let dist = WeightedIndex::new(&weights).unwrap();
        while !rand_nums.is_empty() {
            iter_index[i] += 1;
            let mut rng = rand::thread_rng();
            let weight = dist.sample(&mut rng);
            let num: u64 = rand::thread_rng().gen_range(weight * 100 .. (weight + 1) * 100).try_into().unwrap();
            //check if the random number is in the random numbers vector
            let pos = rand_nums.iter().position(|&x| x == num);

            //if it is, remove it
            //otherwise, continue
            match pos {
                Some(pos) => {
                    let _ = rand_nums.swap_remove(pos);
                },
                None => continue,
            }
        }
    }

    //calculate the min, avg and max iterations it took for each attempt
    let sum: u32 = iter_index.iter().sum();
    let avg = (sum as f32) / (iter_index.len() as f32);
    let min = iter_index.iter().min().unwrap();
    let max = iter_index.iter().max().unwrap();
    println!("Min: {:?}; Avg: {avg}; Max: {:?}", min, max);
}

fn main() {
    let n = read_number("Type a number between 0 and 100",
                        "Could not read the number!",
                        0,
                        100
                        );

    let program = read_number("[1] Do the simple homework\n[2] Do the \"adventurous\" homework",
                              "Could not read the number!",
                              1,
                              2
                             );

    match program {
        1 => simple(n),
        2 => advanced(n),
        _ => panic!("Something terrible happened!"),
    }

}
