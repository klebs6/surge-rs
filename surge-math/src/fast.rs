ix!();

/**
** Fast Math Approximations to various Functions
**
** Documentation on each one
**
** These are directly copied from JUCE6 modules/juce_dsp/mathos/juce_FastMathApproximations.h
**
** Since JUCE6 is GPL3 and Surge is GPL3 that copy is fine, but I did want to explicitly
** acknowledge it
*/

/// JUCE6 Pade approximation of sin valid from -PI to PI 
/// with max error of 1e-5 and average error of 5e-7
#[inline] pub fn fastsin(x: f32) -> f32 
{
    let x2 = x * x;
    let numerator = 
        -x * 
        (
            -11511339840.0 + x2 * 
            (1640635920.0 + x2 * (-52785432.0 + x2 * 479249.0))
        );

    let denominator =
        11511339840.0 
        + x2 * (
            277920720.0 + x2 * (3177720.0 + x2 * 18361.0)
        );

    numerator / denominator
}

/// JUCE6 Pade approximation of cos valid from -PI to PI 
/// with max error of 1e-5 and average error of 5e-7
#[inline] pub fn fastcos(x: f32) -> f32 
{
    let x2 = x * x;

    let numerator = -(-39251520.0 
        + x2 * (18471600.0 + x2 * (-1075032.0 + 14615.0 * x2)));

    let denominator = 
        39251520.0 
        + x2 * (
            1154160.0
            + x2 * (16632.0 + x2 * 127.0));

    numerator / denominator
}

/// Push x into -PI, PI periodically. There is 
/// probably a faster way to do this
#[inline] pub fn clamp_to_pi_range(x: f32) -> f32 
{
    if (-PI_32..=PI_32).contains(&x) {
        return x;
    }

    // so now I am 0,2PI
    let y: f32 = x + PI_32; 

    // float p = fmod( y, 2.0 * PI_32 );

    //one over twopi
    const OO2P: f32 = 1.0 / (2.0 * PI_32);

    let mut p: f32 = y - 2.0 * PI_32 * (((y * OO2P) as i32) as f32);

    if p < 0.0 {
        p += 2.0 * PI_32;
    }

    p - PI_32
}

/// Valid in range -5, 5
#[inline] pub fn fasttanh(x: f32) -> f32 
{
    let x2 = x * x;
    let numerator = x * (135135.0 + x2 * (17325.0 + x2 * (378.0 + x2)));
    let denominator = 135135.0 + x2 * (62370.0 + x2 * (3150.0 + 28.0 * x2));
    numerator / denominator
}

/// Valid in range (-PI/2, PI/2)
#[inline] pub fn fasttan(x: f32) -> f32 
{
    let x2 = x * x;
    let numerator = x * (-135135.0 + x2 * (17325.0 + x2 * (-378.0 + x2)));
    let denominator = -135135.0 + x2 * (62370.0 + x2 * (-3150.0 + 28.0 * x2));
    numerator / denominator
}

#[inline] pub fn fasttanh_sse(x: __m128) -> __m128 
{
    unsafe {
        let m135135: __m128 = _mm_set_ps1(135135.0);
        let m17325:  __m128 = _mm_set_ps1(17325.0);
        let m378:    __m128 = _mm_set_ps1(378.0);
        let m62370:  __m128 = _mm_set_ps1(62370.0);
        let m3150:   __m128 = _mm_set_ps1(3150.0);
        let m28:     __m128 = _mm_set_ps1(28.0);

        let x2 = _mm_mul_ps(x, x);

        let num = _mm_mul_ps(
            x, 
            _mm_add_ps(
                m135135, 
                _mm_mul_ps(
                    x2, 
                    _mm_add_ps(
                        m17325, 
                        _mm_mul_ps(
                            x2, 
                            _mm_add_ps(
                                m378, 
                                x2))))));

        let den = _mm_add_ps(
            m135135, 
            _mm_mul_ps(
                x2, 
                _mm_add_ps(
                    m62370, 
                    _mm_mul_ps(
                        x2, 
                        _mm_add_ps(
                            m3150, 
                            _mm_mul_ps(
                                m28, 
                                x2))))));

        _mm_div_ps(num, den)
    }
}

#[inline] pub fn fasttanh_sse_clamped(x: __m128) -> __m128 
{
    unsafe {
        let xc = _mm_min_ps(
            _mm_set_ps1(5.0), 
            _mm_max_ps(
                _mm_set_ps1(-5.0), 
                x));
        fasttanh_sse(xc)
    }
}

/// Valid in range -6, 4
#[inline] pub fn fastexp(x: f32) -> f32 
{
    let numerator = 1680.0 + x * (840.0 + x * (180.0 + x * (20.0 + x)));
    let denominator = 1680.0 + x * (-840.0 + x * (180.0 + x * (-20.0 + x)));
    numerator / denominator
}

#[inline] pub fn fastexp_sse(x: __m128) -> __m128 
{
    unsafe {
        let m1680:   __m128 = _mm_set_ps1(1680.0);
        let m840:    __m128 = _mm_set_ps1(840.0);
        let mneg840: __m128 = _mm_set_ps1(-840.0);
        let m180:    __m128 = _mm_set_ps1(180.0);
        let m20:     __m128 = _mm_set_ps1(20.0);
        let mneg20:  __m128 = _mm_set_ps1(-20.0);

        let num = _mm_add_ps(
            m1680, 
            _mm_mul_ps(
                x, 
                _mm_add_ps(
                    m840, 
                    _mm_mul_ps(
                        x, 
                        _mm_add_ps(
                            m180, 
                            _mm_mul_ps(
                                x, 
                                _mm_add_ps(
                                    m20, 
                                    x)))))));

        let den = _mm_add_ps(
            m1680, 
            _mm_mul_ps(
                x, 
                _mm_add_ps(
                    mneg840, 
                    _mm_mul_ps(
                        x, 
                        _mm_add_ps(
                            m180, 
                            _mm_mul_ps(
                                x, 
                                _mm_add_ps(
                                    mneg20, 
                                    x)))))));

        _mm_div_ps(num, den)
    }
}
