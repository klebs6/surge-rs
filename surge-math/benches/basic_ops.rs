#![feature(test)]
#[macro_use]
extern crate criterion;
extern crate test;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use surge_math::{add_block, accumulate_block, mul_block, subtract_block};

fn bench_mul_block(c: &mut Criterion) {
    let mut src1 = [1.0; 1024];
    let mut src2 = [2.0; 1024];
    let mut dst = [0.0; 1024];

    c.bench_function("mul_block", |b| {
        b.iter(|| mul_block(black_box(src1.as_mut_ptr()), black_box(src2.as_mut_ptr()), black_box(dst.as_mut_ptr()), black_box(256)))
    });
}

fn bench_add_block(c: &mut Criterion) {
    let mut src1 = [1.0; 1024];
    let mut src2 = [2.0; 1024];
    let mut dst = [0.0; 1024];

    c.bench_function("add_block", |b| {
        b.iter(|| add_block(black_box(src1.as_mut_ptr()), black_box(src2.as_mut_ptr()), black_box(dst.as_mut_ptr()), black_box(256)))
    });
}

fn bench_subtract_block(c: &mut Criterion) {
    let mut src1 = [1.0; 1024];
    let mut src2 = [2.0; 1024];
    let mut dst = [0.0; 1024];

    c.bench_function("subtract_block", |b| {
        b.iter(|| subtract_block(black_box(src1.as_mut_ptr()), black_box(src2.as_mut_ptr()), black_box(dst.as_mut_ptr()), black_box(256)))
    });
}

fn bench_accumulate_block(c: &mut Criterion) {
    let mut src = [1.0; 1024];
    let mut dst = [0.0; 1024];

    c.bench_function("accumulate_block", |b| {
        b.iter(|| accumulate_block(black_box(src.as_mut_ptr()), black_box(dst.as_mut_ptr()), black_box(256)))
    });
}

criterion_group!(
    benches, 
    bench_mul_block, 
    bench_add_block, 
    bench_subtract_block, 
    bench_accumulate_block
);

criterion_main!(basic_ops);
