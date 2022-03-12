ix!();

#[inline] pub fn sincf(x: f64) -> f64 {

    if x == 0.0 {
        return 1.0;
    }

    (PI * x).sin() / (PI * x)
}

#[inline] pub fn sinc(x: f64) -> f64 {

    if x.abs() < 0.0000000000000000000001 {
        1.0
    } else {
        x.sin() / x
    }
}

