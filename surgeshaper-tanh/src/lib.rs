#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use surge_filter::Waveshaper;

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

            let mut y: __m128 = _mm_mul_ps(x, _mm_add_ps(m27, xx));

            y = _mm_mul_ps(y, _mm_rcp_ps(denom));

            let y_min: __m128 = _mm_set1_ps(-1.0);
            let y_max: __m128 = _mm_set1_ps(1.0);
            _mm_max_ps(_mm_min_ps(y, y_max), y_min)
        }
    }
}

#[test] fn smoke() {
    use surge_math::{m128_one, m128_half};

    let shaper = TanhShaper::default();

    let result = shaper.shape(m128_one![], m128_half![]);
    println!("shaper: {:?}, result: {:?}", shaper, result);

    let result = shaper.shape(result, m128_half![]);
    println!("shaper: {:?}, result: {:?}", shaper, result);
}
