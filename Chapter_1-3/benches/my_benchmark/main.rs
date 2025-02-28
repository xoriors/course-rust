use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use chapter_1_3::weighted::wei_dist::weighted_distribution;
use chapter_1_3::uniform::uni_dist::uniform_distribution;

fn bench_distributions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distribution Comparison");

    let mut extracted_array_u64 = [123, 235, 633, 677, 888, 324];
    let mut extracted_array_f32 = [ 0.54633904, 0.506454, 0.10968602, 0.24843967, 0.4981829, 0.43807256];
    let matched = [false; 100];
    let success = 0;
    let user_input = 6;

    for &loop_amount in &[10u64, 100u64, 1000u64] {
        group.bench_with_input(
            BenchmarkId::new("WEIGHTED-DISTRIBUTION", loop_amount),
            &loop_amount,
            |b, &loop_amount| {
                b.iter(|| weighted_distribution(loop_amount, &mut extracted_array_u64, matched))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("UNIFORM-DISTRIBUTION", loop_amount),
            &loop_amount,
            |b, &loop_amount| {
                b.iter(|| uniform_distribution(loop_amount, user_input, &mut extracted_array_f32, success, matched))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_distributions);
criterion_main!(benches);
