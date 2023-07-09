#![feature(test)]
#[macro_use]
extern crate criterion;
extern crate test;

use criterion::{black_box, Criterion};
use surge_math::{clamp01, clamp1_bipolar, within_range};

fn bench_clamp(c: &mut Criterion) {

    c.bench_function("clamp01", |b| {
        b.iter(|| {
            clamp01(black_box(1.5));
        })
    });

    c.bench_function("clamp1_bipolar", |b| {
        b.iter(|| {
            clamp1_bipolar(black_box(1.5));
        })
    });

    c.bench_function("within_range", |b| {
        b.iter(|| {
            within_range(black_box(1), black_box(2), black_box(3));
        })
    });
}

criterion_group!(clamp_benches, bench_clamp);

criterion_main!(clamp_benches);
