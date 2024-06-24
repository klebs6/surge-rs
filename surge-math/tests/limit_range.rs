use surge_math::*;
use surge_imports::*;

#[traced_test]
fn test_limit_range_f32() -> Result<(),MathError> {
    assert_eq!(limit_range(0.0f32, -1.0, 1.0), 0.0);
    assert_eq!(limit_range(-2.0f32, -1.0, 1.0), -1.0);
    assert_eq!(limit_range(2.0f32, -1.0, 1.0), 1.0);

    // Test SIMD path specifically
    unsafe {
        let x = 0.0f32;
        let low = -1.0f32;
        let high = 1.0f32;

        let _s = _mm_load_ss(&x);
        let _low = _mm_load_ss(&low);
        let _high = _mm_load_ss(&high);
        let limited = _mm_min_ss(_mm_max_ss(_s, _low), _high);
        let mut result = 0.0f32;
        _mm_store_ss(&mut result, limited);

        assert_eq!(result, 0.0);
    }

    Ok(())
}

#[traced_test] fn test_limit_range_f32_nosimd() -> Result<(),MathError> {
    assert_eq!(limit_range(0.0f32, -1.0, 1.0), 0.0);
    assert_eq!(limit_range(-2.0f32, -1.0, 1.0), -1.0);
    assert_eq!(limit_range(2.0f32, -1.0, 1.0), 1.0);

    // Using the non-SIMD path directly
    let x = 0.0f32;
    let low = -1.0f32;
    let high = 1.0f32;
    let result = x.limit_range_nosimd(low, high);
    assert_eq!(result, 0.0);

    Ok(())
}

#[traced_test] fn test_limit_range_i32() -> Result<(),MathError> {
    assert_eq!(limit_range(0i32, -1, 1), 0);
    assert_eq!(limit_range(-2i32, -1, 1), -1);
    assert_eq!(limit_range(2i32, -1, 1), 1);

    Ok(())
}

#[traced_test] fn test_limit_range_u32() -> Result<(),MathError> {
    assert_eq!(limit_range(0u32, 1, 3), 1);
    assert_eq!(limit_range(2u32, 1, 3), 2);
    assert_eq!(limit_range(4u32, 1, 3), 3);

    Ok(())
}

#[traced_test] fn test_limit_range_f64() -> Result<(),MathError> {
    assert_abs_diff_eq!(limit_range(0.0f64, -1.0, 1.0), 0.0, epsilon = 1e-6);
    assert_abs_diff_eq!(limit_range(-2.0f64, -1.0, 1.0), -1.0, epsilon = 1e-6);
    assert_abs_diff_eq!(limit_range(2.0f64, -1.0, 1.0), 1.0, epsilon = 1e-6);

    Ok(())
}

#[traced_test] fn test_limit_range_f64_nosimd() -> Result<(),MathError> {
    let x = 0.0f64;
    let low = -1.0f64;
    let high = 1.0f64;
    let result = x.limit_range_nosimd(low, high);
    assert_abs_diff_eq!(result, 0.0, epsilon = 1e-6);

    let x = -2.0f64;
    let low = -1.0f64;
    let high = 1.0f64;
    let result = x.limit_range_nosimd(low, high);
    assert_abs_diff_eq!(result, -1.0, epsilon = 1e-6);

    let x = 2.0f64;
    let low = -1.0f64;
    let high = 1.0f64;
    let result = x.limit_range_nosimd(low, high);
    assert_abs_diff_eq!(result, 1.0, epsilon = 1e-6);

    Ok(())
}
