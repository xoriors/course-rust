use rand::Rng;
use std::{io, time};

fn main() {
    let number = loop {
        println!("Please input your a number.");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    let mut loop_iteration_counter = 0u64;
    const NUMBER_OF_RUNS: usize = 100;
    let mut times = [0.0; NUMBER_OF_RUNS];
    let mut rng = rand::rng();
    for i in 0..NUMBER_OF_RUNS {
        let now = time::Instant::now();
        let mut vector_of_random_numbers: Vec<f32> = Vec::new();
        for _ in 0..number {
            let random_float: f32 = rng.random();
            vector_of_random_numbers.push(random_float);
        }
        dbg!(vector_of_random_numbers.len());
        dbg!(&vector_of_random_numbers);
        loop {
            loop_iteration_counter += 1;
            let random_float: f32 = rng.random();
            if vector_of_random_numbers.len() == 0 {
                break;
            }
            match vector_of_random_numbers
                .iter()
                .position(|number_in_vec| random_float == *number_in_vec)
            {
                Some(index) => vector_of_random_numbers.remove(index),
                None => continue,
            };
        }
        times[i] = now.elapsed().as_secs_f64();
        println!(
            "Instance: {}\tIterations: {}\tTime taken: {}s",
            i,
            loop_iteration_counter,
            now.elapsed().as_secs_f64()
        );
    }
    dbg!(&times);
    dbg!(times.iter().sum::<f64>() / NUMBER_OF_RUNS as f64);
}
