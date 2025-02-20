use std::io;
use rand::Rng;

struct Result {
    size: u8,
    min: u64,
    max: u64,
    avg: f64,
}

fn main() {
    for size in 1..100 { // fiecare lungime
        println!("Size: {}", size);
        let mut iter_result = Result {
            size: size,
            min: 0,
            max: 0,
            avg: 0.0,
        };

        for _ in 0..100 { // fiecare iteratie
            let n: usize = size.into();
            let mut array:Vec<f32> = vec![0.0; n];

            for i in 0..n {
                array[i] = rand::thread_rng().gen_range(0.0..1.0);
            }

            let mut counter = 0;

            loop {
                counter += 1;
                let num = rand::thread_rng().gen_range(0.0..1.0);
                if array.contains(&num) {
                    let index = array.iter().position(|&x| x == num).unwrap();
                    array.remove(index);
                    // println!("Removed: {num} on iter {counter}, array is now {array:?}");
                    if array.is_empty() {
                        break;
                    }
                }
            }

            if counter < iter_result.min || iter_result.min == 0 {
                iter_result.min = counter;
            }

            if counter > iter_result.max {
                iter_result.max = counter;
            }

            iter_result.avg += counter as f64;
        }
        println!("Min: {}, Max: {}, Avg: {}", iter_result.min, iter_result.max, iter_result.avg / 100.0);
    }
}