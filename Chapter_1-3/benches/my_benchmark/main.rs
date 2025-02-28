use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use chapter_1_3::weighted::wei_dist::weighted_distribution;
use chapter_1_3::uniform::uni_dist::uniform_distribution;
use rand::random_range;

fn bench_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distribution Comparison");

    let matched = [false; 100];
    let success = 0;
    let loop_amount = 0;

    for &array_size in &[4, 8, 12] {
        let mut extracted_array_u64 = vec![0; array_size];  // Adjust size
        let mut extracted_array_f32 = vec![0.0; array_size]; // Adjust size
        for i in extracted_array_u64.iter_mut() {
            let generated_u64 = random_range(0..=1000);
            *i = generated_u64
        };
        for i in extracted_array_f32.iter_mut() {
            let generated_f32 = random_range(0.0..=1.0);
            *i = generated_f32
        };
        group.bench_with_input(
            BenchmarkId::new("WEIGHTED-DISTRIBUTION", array_size),
            &array_size,
            |b, &_| {
                b.iter(|| weighted_distribution(loop_amount, &mut extracted_array_u64, matched))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("UNIFORM-DISTRIBUTION", array_size),
            &array_size,
            |b, &array_size| {
                b.iter(|| uniform_distribution(loop_amount, array_size as u16, &mut extracted_array_f32, success, matched))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_distributions);
criterion_main!(benches);
