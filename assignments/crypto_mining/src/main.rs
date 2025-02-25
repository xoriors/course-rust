use std::io;
use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;
use std::collections::HashMap;

const BILLION: u64 = 1_000_000_000;

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
            let mut rng = rand::thread_rng();

            let mut array:Vec<u64> = vec![0; n];
            // let mut weights:Vec<u64> = vec![0; (BILLION / 100) as usize];
            // let mut weights:Vec<u64, u8> = vec![(0, 0); 1];
            let mut weight_map = HashMap::new();

            for i in 0..n {
                array[i] = rng.gen_range(0..BILLION);
                if weight_map.contains_key(&(array[i] / 100)) {
                    *weight_map.get_mut(&(array[i] / 100)).unwrap() += 1;
                } else {
                    weight_map.insert(array[i] / 100, 1);
                }
            }

            let mut keys:Vec<u64> = weight_map.clone().into_keys().collect();
            let mut weights:Vec<u64> = weight_map.into_values().collect();

            // println!("{:?} \n {:?}", keys, weights);


            // println!("Weights: {dist:?}");
            // println!("Array: {array:?}");

            let mut counter = 0;

            loop {
                let mut dist = WeightedIndex::new(&weights).unwrap();
                counter += 1;
                let random_index = dist.sample(&mut rng);
                // println!("{random_index}");
                let mut num: u64 = (keys[random_index] * 100 + rng.gen_range(0..100)).try_into().unwrap();

                // small hack -> weighted gen pt num / 100 + random gen 0-100

                if array.contains(&num) {
                    let index = array.iter().position(|&x| x == num).unwrap();
                    array.remove(index);
                    // println!("Removed: {num} on iter {counter}, array is now {array:?}");
                    if array.is_empty() {
                        break;
                    }
                    weights[random_index] -= 1;
                    if weights[random_index] == 0 {
                        weights.remove(random_index);
                        keys.remove(random_index);
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

            // break;
        }
        println!("Min: {}, Max: {}, Avg: {}", iter_result.min, iter_result.max, iter_result.avg / 100.0);
    }
}