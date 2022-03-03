ix!();

pub fn flush_denormal(d: &mut f64) {
    if d.abs() < 1e-30 {
        *d = 0.0;
    }
}

///utility to split a floating point number 
///into integral and fractional parts
#[inline] 
pub fn split_float(f: f32) -> (f32, f32) {

    if f == 0.0 {
        return (0.0, 0.0);
    }

    let i = unsafe{ f.to_int_unchecked::<i32>() };

    // (integral, fractional)
    (i as f32, f - i as f32) 
}

