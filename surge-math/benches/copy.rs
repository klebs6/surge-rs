use super::*;
use criterion::{criterion_group, criterion_main, Criterion};
use std::mem;

fn bench_copy_block_func(
    c: &mut Criterion,
    name: &str,
    copy_func: fn(*mut f32, *mut f32, usize),
) {
    let src = generate_aligned_f32_vec(1024);
    let mut dst = vec![0.0; 1024];

    c.bench_function(name, |b| {
        b.iter(|| {
            test_copy_block_func(copy_func, &src, &mut dst);
        })
    });
}

fn copy_benchmark(c: &mut Criterion) {
    bench_copy_block_func(c, "copy_block", copy_block);
    bench_copy_block_func(c, "copy_block_unaligned_source", copy_block_unaligned_source);
    bench_copy_block_func(c, "copy_block_unaligned_destination", copy_block_unaligned_destination);
    bench_copy_block_func(c, "copy_block_unaligned_src_and_dst", copy_block_unaligned_src_and_dst);
}

criterion_group!(benches, copy_benchmark);
criterion_main!(benches);

fn generate_aligned_f32_vec(len: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen_range(-1.0..1.0)).collect()
}
