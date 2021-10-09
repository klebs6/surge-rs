#[bench]
fn simd_intermediates(b: &mut test::Bencher) {

    unsafe {
        use core::arch::x86_64::*;

        let s1  = _mm_set1_ps(5.0);
        let s2  = _mm_set1_ps(1.0);
        let s3  = _mm_set1_ps(15.0);
        let s4  = _mm_set1_ps(-5.5);
        let ml  = _mm_set1_ps(0.6);

        let worker = |f| {
            let x = _mm_set1_ps(f as f32);
            let _0 = _mm_mul_ps(x, s1);
            let _1 = _mm_add_ps(_0,  s2);
            let _2 = _mm_mul_ps(x, _1);
            let _3 = _mm_add_ps(_2,  s3);
            let _4 = _mm_mul_ps(x, _3);
            let _5 = _mm_add_ps(_4,  s4);
            _mm_mul_ps(_5, ml)
        };

        // exact code to benchmark must be passed as a closure to the iter
        // method of Bencher
        b.iter(|| {
            (0..200).map(worker).collect::<Vec<__m128>>()
        });
    }
}

#[bench]
fn simd_no_intermediates(b: &mut test::Bencher) {

    unsafe {
        use core::arch::x86_64::*;

        let s1  = _mm_set1_ps(5.0);
        let s2  = _mm_set1_ps(1.0);
        let s3  = _mm_set1_ps(15.0);
        let s4  = _mm_set1_ps(-5.5);
        let ml  = _mm_set1_ps(0.6);

        let worker = |f| {
            let x = _mm_set1_ps(f as f32);

            _mm_mul_ps(
                _mm_add_ps(
                    _mm_mul_ps(
                        x, 
                        _mm_add_ps(
                            _mm_mul_ps(
                                x, 
                                _mm_add_ps(
                                    _mm_mul_ps(
                                        x, 
                                        s1), 
                                    s2)), 
                            s3)), 
                    s4), 
                ml)
        };

        // exact code to benchmark must be passed as a closure to the iter
        // method of Bencher
        b.iter(|| {
            (0..200).map(worker).collect::<Vec<__m128>>()
        });
    }
}

