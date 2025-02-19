use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cryptoMiningAssignment::random_removal;

fn benchmark(c: &mut Criterion) {
    c.bench_function("random_removal 50", |b| b.iter(|| random_removal(black_box(50))));
    c.bench_function("random_removal 100", |b| b.iter(|| random_removal(black_box(100))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
