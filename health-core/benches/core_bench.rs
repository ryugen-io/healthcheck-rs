use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("noop", |b| b.iter(|| 1));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
