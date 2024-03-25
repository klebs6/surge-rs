use surgeshaper_tanh::*;
use surge_filter::Waveshaper;

#[test] fn smoke() {

    use surge_math::simd_m128;

    let shaper = TanhShaper::default();

    let result = shaper.shape(simd_m128::one(), simd_m128::half());
    println!("shaper: {:?}, result: {:?}", shaper, result);

    let result = shaper.shape(result, simd_m128::half());
    println!("shaper: {:?}, result: {:?}", shaper, result);
}
