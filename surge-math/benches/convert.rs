#![feature(test)]
#[macro_use]
extern crate criterion;
extern crate test;

use criterion::{criterion_group, criterion_main, Criterion};
use surge_math::*;

fn bench_amp_linear(c: &mut Criterion) {
    let mut x = 0.5;
    c.bench_function("amp_linear", |b| {
        b.iter(|| {
            x = amp_to_linear(x)
        })
    });
}

fn bench_linear_to_amp(c: &mut Criterion) {
    let mut x = 0.5;
    c.bench_function("linear_to_amp", |b| {
        b.iter(|| {
            x = linear_to_amp(x)
        })
    });
}

fn bench_amp_db(c: &mut Criterion) {
    let mut x = 0.5;
    c.bench_function("amp_db", |b| {
        b.iter(|| {
            x = amp_to_db(x)
        })
    });
}

fn bench_db_to_amp(c: &mut Criterion) {
    let mut x = 0.5;
    c.bench_function("db_to_amp", |b| {
        b.iter(|| {
            x = db_to_amp(x)
        })
    });
}

criterion_group!(
    benches,
    bench_amp_linear,
    bench_linear_to_amp,
    bench_amp_db,
    bench_db_to_amp
);

criterion_main!(benches);
