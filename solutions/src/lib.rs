
use rand::Rng;

pub fn random_removal(n: usize) -> usize {
    let mut rng = rand::rng();
    let mut numbers: Vec<f32> = (0..n).map(|_| rng.random_range(0.0..1.0)).collect();
    let mut count = 0;

    while !numbers.is_empty() {
        count += 1;
        let rand_num = rng.random_range(0.0..1.0);
        numbers.retain(|&x| (x - rand_num).abs() > 1e-5);
    }

    count
}
