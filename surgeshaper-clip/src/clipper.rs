crate::ix!();

#[derive(Debug,Default)]
pub struct Clipper {}

#[cfg(target_arch = "x86_64")] 
impl Waveshaper for Clipper {

    fn shape(&self, input: __m128, drive: __m128) -> __m128 {
        unsafe {
            clip_bipolar(_mm_mul_ps(input, drive))
        }
    }
}

#[test] fn smoke() {

    use surge_math::simd_m128;

    let shaper = Clipper::default();

    let result = shaper.shape(simd_m128::one(), simd_m128::half());
    println!("shaper: {:?}, result: {:?}", shaper, result);

    let result = shaper.shape(result, simd_m128::half());
    println!("shaper: {:?}, result: {:?}", shaper, result);
}

