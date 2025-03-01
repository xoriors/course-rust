use core::f32;
use rand::{rngs::ThreadRng, Rng};
use std::io;
use std::time::Instant;

fn main() {
    let user_input: i32;
    loop {
        let mut guess = String::new();
        println!("Please enter array size: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        match guess.trim().parse() {
            Ok(x) => {
                if x < 1 || x > 100 {
                    println!("number must be between 1 and 100");
                    continue;
                } else {
                    user_input = x;
                    break;
                }
            }
            Err(error) => {
                println!("error when parsing number: {error}");
                continue;
            }
        };
    }
    println!("using input {user_input}");

    let mut rand_nums: Vec<(f32, bool)> = Vec::new();
    let mut rng = rand::rng();
    for _ in 0..user_input {
        rand_nums.push((rng.random_range(0.0..1.0), false));
    }

    const REPETITIONS: usize = 10;
    let mut break_durations: [f32; REPETITIONS] = [0.0; REPETITIONS];
    for i in 0..REPETITIONS {
        let d = break_nums(&rand_nums, &mut rng);
        println!("took {} seconds to guess everything", d);
        break_durations[i] = d
    }
    let mut max = f32::NEG_INFINITY;
    let mut min = f32::INFINITY;
    let mut avg = 0.0;

    for d in break_durations {
        if d > max {
            max = d;
        }
        if d < min {
            min = d;
        }
        avg += d;
    }
    println!("min {}, max {}, avg {}", min, max, avg / REPETITIONS as f32);
}

fn break_nums(rand_nums: &Vec<(f32, bool)>, rng: &mut ThreadRng) -> f32 {
    let now = Instant::now();

    let mut rand_nums = rand_nums.clone();
    let mut cnt = 0;
    let size: i32 = rand_nums
        .len()
        .try_into()
        .expect("size should be between 1 and 100");
    loop {
        let num: f32 = rng.random_range(0.0..1.0);
        match rand_nums
            .iter_mut()
            .find(|(n, erased)| *n == num && !erased)
        {
            Some(found) => {
                found.1 = true;
                cnt += 1;
                println!("erased {}", found.0);
                if cnt == size {
                    break;
                }
            }
            None => {}
        }
    }

    now.elapsed().as_secs_f32()
}
