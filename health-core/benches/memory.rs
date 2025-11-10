use criterion::{Criterion, black_box, criterion_group, criterion_main};
use healthcheck_core::memory::{parse_cgroup_bytes, parse_meminfo_value};

fn bench_parse_meminfo(c: &mut Criterion) {
    let sample = "MemTotal:    123456 kB";
    c.bench_function("parse_meminfo_value", |b| {
        b.iter(|| parse_meminfo_value(black_box(sample)))
    });
}

fn bench_parse_cgroup_bytes(c: &mut Criterion) {
    let sample = "1048576";
    c.bench_function("parse_cgroup_bytes", |b| {
        b.iter(|| parse_cgroup_bytes(black_box(sample)))
    });
}

criterion_group!(
    memory_benches,
    bench_parse_meminfo,
    bench_parse_cgroup_bytes
);
criterion_main!(memory_benches);
