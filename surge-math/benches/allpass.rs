#![feature(test)]
#[macro_use]
extern crate criterion;
extern crate test;

use criterion::{black_box, Criterion};
use surge_math::AllpassFilter;

/// This benchmark will run the `process` method on the
/// `AllpassFilter` with a buffer size of 256 and a filter
/// coefficient of 0.3. 
///
/// The `black_box` function is used to prevent the compiler
/// from optimizing away the benchmark code.
///
fn bench_allpass_filter(c: &mut Criterion) {
    c.bench_function("allpass_filter", |b| {
        let mut allpass_filter = AllpassFilter::<256>::default();
        allpass_filter.set_a(0.3);

        b.iter(|| {
            for _ in 0..1024 {
                allpass_filter.process(black_box(0.5));
            }
        })
    });
}

criterion_group!(allpass_benches, bench_allpass_filter);
criterion_main!(allpass_benches);
