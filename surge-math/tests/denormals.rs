use surge_math::*;

#[test] fn test_flush_denormal() {

    let mut f: f64 = 1e-31;
    flush_denormal(&mut f);
    assert!(f == 0.0);

    let mut f2: f64 = 1e-29;
    flush_denormal(&mut f2);
    assert!(f2 != 0.0);
}
