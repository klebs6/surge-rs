#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use surge_filter::Waveshaper;

#[derive(Debug,Default)]
pub struct Clipper {}

#[cfg(target_arch = "x86_64")] 
impl Waveshaper for Clipper {

    fn shape(&self, input: __m128, drive: __m128) -> __m128 {
        unsafe {
            let x_min: __m128 = _mm_set1_ps(-1.0);
            let x_max: __m128 = _mm_set1_ps(1.0);
            _mm_max_ps(
                _mm_min_ps(
                    _mm_mul_ps(
                        input, 
                        drive), 
                    x_max), 
                x_min)
        }
    }
}

#[test] fn smoke() {

    use surge_math::{m128_one,m128_half};

    let shaper = Clipper::default();

    let result = shaper.shape(m128_one![], m128_half![]);
    println!("shaper: {:?}, result: {:?}", shaper, result);

    let result = shaper.shape(result, m128_half![]);
    println!("shaper: {:?}, result: {:?}", shaper, result);
}
