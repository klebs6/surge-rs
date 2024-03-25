use surge_math::*;
use surge_imports::assert_approx_eq;

#[test] fn test_lerp() {
    let result = lerp(0.25, 5.0, 7.0);
    assert_approx_eq!(result, 5.5);
}

#[test]
fn test_clamp1_bipolar() {
    assert_eq!(clamp1_bipolar(-1.5), -1.0);
    assert_eq!(clamp1_bipolar(0.5), 0.5);
    assert_eq!(clamp1_bipolar(1.5), 1.0);
}

#[test]
fn test_within_range() {
    assert!(within_range(1, 2, 3));
    assert!(!within_range(1, 0, 3));
    assert!(!within_range(1, 4, 3));
    assert!(within_range(3,5,10));
    assert!(!within_range(3,15,10));
}

#[test] fn test_clamp01() {

    assert_eq!(clamp01(-0.5), 0.0);
    assert_eq!(clamp01(0.5), 0.5);
    assert_eq!(clamp01(1.5), 1.0);

    let x: f64 = 1e-31;
    assert!(clamp01(x) == 1e-31);

    let x: f64 = -0.3;
    assert!(clamp01(x) == 0.0);

    let x: f64 = 1.3;
    assert!(clamp01(x) == 1.0);
}

#[test] fn test_clamp1bp() {

    let x: f64 = 1e-31;
    assert!(clamp1_bipolar(x) == 1e-31);

    let x: f64 = -0.3;
    assert!(clamp1_bipolar(x) == -0.3);

    let x: f64 = 1.3;
    assert!(clamp1_bipolar(x) == 1.0);
}
