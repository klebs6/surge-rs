crate::ix!();

#[derive(Default,Debug)]
pub struct TanhShaper { }

impl Waveshaper for TanhShaper {

    #[cfg(target_arch = "x86_64")] 
    fn shape(&self, input: __m128, drive: __m128) -> __m128 {

        // Closer to ideal than TANH0
        // y = x * ( 27 + x * x ) / ( 27 + 9 * x * x );
        // y = clip(y)

        unsafe { 
            let m9:    __m128 = _mm_set1_ps(9.0);
            let m27:   __m128 = _mm_set1_ps(27.0);

            let x:     __m128 = _mm_mul_ps(input, drive);
            let xx:    __m128 = _mm_mul_ps(x, x);
            let denom: __m128 = _mm_add_ps(m27, _mm_mul_ps(m9, xx));

            let y: __m128 = {
                let numerator = _mm_mul_ps(x, _mm_add_ps(m27, xx));
                _mm_mul_ps(numerator, _mm_rcp_ps(denom))
            };

            clip_bipolar(y)
        }
    }
}

#[test] fn smoke() {
    use surge_math::simd_m128;

    let shaper = TanhShaper::default();

    let result = shaper.shape(simd_m128::one(), simd_m128::half());
    println!("shaper: {:?}, result: {:?}", shaper, result);

    let result = shaper.shape(result, simd_m128::half());
    println!("shaper: {:?}, result: {:?}", shaper, result);
}

