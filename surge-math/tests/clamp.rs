use surge_math::*;

#[test]
fn test_clamp01() {
    assert_eq!(clamp01(-0.5), 0.0);
    assert_eq!(clamp01(0.5), 0.5);
    assert_eq!(clamp01(1.5), 1.0);
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
}
