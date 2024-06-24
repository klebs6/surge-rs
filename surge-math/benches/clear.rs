#![feature(test)]
#[macro_use]
extern crate criterion;
extern crate test;

use criterion::{black_box, Criterion};
use std::mem::MaybeUninit;

#[cfg(target_feature = "sse")]
fn bench_functions(c: &mut Criterion) {
    c.bench_function("clear_block", |b| {
        const NQUADS: usize = 4;
        let mut input: [f32; NQUADS * 4] = [1.0; NQUADS * 4];
        b.iter(|| {
            unsafe { clear_block::<NQUADS>(black_box(input.as_mut_ptr())).unwrap() };
        })
    });

    c.bench_function("clear_block_antidenormalnoise", |b| {
        const NQUADS: usize = 4;
        let mut input: [f32; NQUADS * 4] = [1.0; NQUADS * 4];
        b.iter(|| {
            unsafe {
                clear_block_antidenormalnoise::<NQUADS>(black_box(input.as_mut_ptr()))
            };
        })
    });
}

#[cfg(target_feature = "sse")]
criterion_group!(function_benches, bench_functions);

#[cfg(target_feature = "sse")]
criterion_main!(function_benches);
