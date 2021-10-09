ix!();

#[cfg(test)]
mod clamp_benches {
    use crate::imports::*;
    use test::Bencher;
    import_all!();
    #[bench] fn bench_limit_range_f(b: &mut Bencher) {
        b.iter(|| {
            let x: f32 = 100.0 * rand11();
            let min: f32 = -25.0;
            let max: f32 = 25.0;
            let y = limit_range_f(x,min,max);
        })
    }
    #[bench] fn bench_limit_range_f_nosimd(b: &mut Bencher) {
        b.iter(|| {
            let x: f32 = 100.0 * rand11();
            let min: f32 = -25.0;
            let max: f32 = 25.0;
            let y = limit_range_f_nosimd(x,min,max);
        })
    }
    #[bench] fn bench_limit_range_i(b: &mut Bencher) {
        b.iter(|| {
            let x: i32 = (100.0 * rand11()) as i32;
            let min: i32 = -25;
            let max: i32 = 25;
            let y = limit_range_i(x,min,max);
        })
    }
}

