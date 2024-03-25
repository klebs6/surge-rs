/*
crate::ix!();

/// To test this code, you would want to write
/// unit tests that cover various input scenarios
/// and ensure that the output coefficients are
/// correct. Here's an example of a unit test for
/// this implementation:
///
/// This test creates a `BiquadFilter` instance
/// and calls `coeff_lph_morph` with different
/// input parameters. It then gets the filter
/// coefficients using the `get_coef` method and
/// checks that they match the expected values.
///
#[test]
fn test_coeff_lph_morph() {

    let mut filter = BiquadFilter::new();

    // Test with omega = 1.0, quality_factor = 0.5, and morph = 0.0
    filter.coeff_lph_morph(1.0, 0.5, 0.0);
    let (a0, a1, a2, b0, b1, b2) = filter.get_coef();
    assert_eq!(a0, 1.1913074589906563);
    assert_eq!(a1, -1.7788737322075763);
    assert_eq!(a2, 0.7886435410093439);
    assert_eq!(b0, 0.09565372949532816);
    assert_eq!(b1, 0.0);
    assert_eq!(b2, -0.09565372949532816);

    // Test with omega = 0.5, quality_factor = 0.8, and morph = 0.5
    filter.coeff_lph_morph(0.5, 0.8, 0.5);
    let (a0, a1, a2, b0, b1, b2) = filter.get_coef();
    assert_eq!(a0, 1.2360679774997896);
    assert_eq!(a1, -0.6755902076157204);
    assert_eq!(a2, 0.2639320225002104);
    assert_eq!(b0, 0.4539394232516931);
    assert_eq!(b1, 0.0);
    assert_eq!(b2, -0.4539394232516931);
}

*/
