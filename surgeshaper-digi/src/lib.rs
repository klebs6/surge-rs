#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use surge_filter::Waveshaper;

#[derive(Debug,Default)]
pub struct DigiShaper {}

#[cfg(target_arch = "x86_64")] 
impl Waveshaper for DigiShaper {
    fn shape(&self, input: __m128, drive: __m128) -> __m128 {
        // v1.2: return (double)((int)((double)(x*p0inv*16.f+1.0)))*p0*0.0625f;
        unsafe {
            let m16:    __m128 = _mm_set1_ps(16.0);
            let m16inv: __m128 = _mm_set1_ps(0.0625);
            let mofs:   __m128 = _mm_set1_ps(0.5);

            let invdrive: __m128 = _mm_rcp_ps(drive);
            let a: __m128i = _mm_cvtps_epi32(_mm_add_ps(mofs, _mm_mul_ps(invdrive, _mm_mul_ps(m16, input))));

            _mm_mul_ps(drive, _mm_mul_ps(m16inv, _mm_sub_ps(_mm_cvtepi32_ps(a), mofs)))
        }
    }
}

#[cfg(not(target_arch = "x86_64"))] 
impl Waveshaper for DigiShaper {

    fn shape(&mut self, input: __m128, drive: __m128) -> __m128 {
        unsafe {
            let mofs:   __m128 = _mm_set1_ps(0.5);
            let m16:    __m128 = _mm_set1_ps(16.0);
            let m16inv: __m128 = _mm_set1_ps(0.0625);

            let invdrive: __m128 = _mm_rcp_ps(drive);
            let mut a:    __m128 = _mm_add_ps(mofs, _mm_mul_ps(invdrive, _mm_mul_ps(m16, input)));

            let a1: __m64 = _mm_cvtps_pi32(a);
            let a2: __m64 = _mm_cvtps_pi32(_mm_movehl_ps(a, a));

            a = _mm_cvtpi32_ps(_mm_movelh_ps(a, _mm_cvtpi32_ps(a, a2)), a1);

            let b: __m128  = _mm_mul_ps(drive, _mm_mul_ps(m16inv, _mm_sub_ps(a, mofs)));

            _mm_empty();

            b
        }
    }
}

#[test] fn smoke() {

    use surge_math::{m128_one,m128_half};

    let shaper = DigiShaper::default();

    let one  = unsafe { m128_one!{} };
    let half = unsafe { m128_half!{} };

    let result = shaper.shape(one, half);
    println!("shaper: {:?}, result: {:?}", shaper, result);

    let result = shaper.shape(result, half);
    println!("shaper: {:?}, result: {:?}", shaper, result);
}
