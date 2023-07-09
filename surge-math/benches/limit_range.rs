crate::ix!();

use crate::*;
use bencher::Bencher;

fn bench_limit_range(b: &mut Bencher) {
    b.iter(|| {
        let x: f32 = 100.0 * rand11();
        let min: f32 = -25.0;
        let max: f32 = 25.0;
        let y = limit_range(x,min,max);
    })
}

fn bench_limit_range_nosimd(b: &mut Bencher) {
    b.iter(|| {
        let x: f32 = 100.0 * rand11();
        let min: f32 = -25.0;
        let max: f32 = 25.0;
        let y = x.limit_range_nosimd(min,max);
    })
}

fn bench_limit_range_i(b: &mut Bencher) {
    b.iter(|| {
        let x: i32 = (100.0 * rand11()) as i32;
        let min: i32 = -25;
        let max: i32 = 25;
        let y = limit_range(x,min,max);
    })
}

benchmark_group!(
    benches, 
    bench_limit_range, 
    bench_limit_range_nosimd,
    bench_limit_range_i
);

benchmark_main!(benches);
