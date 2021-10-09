ix!();

use crate::MyFloat;

#[inline] 
pub fn cubic_interpolate(
    y0: f32,
    y1: f32,
    y2: f32,
    y3: f32,
    mu: f32) -> f32 
{
    let mu2: f32 = mu * mu;
    let a0: f32  = y3 - y2 - y0 + y1;
    let a1: f32  = y0 - y1 - a0;
    let a2: f32  = y2 - y0;
    let a3: f32  = y1;

    a0 * mu * mu2 + a1 * mu2 + a2 * mu + a3
}

#[inline] 
pub fn lerp<T: MyFloat>(x: T, a: T, b: T) -> T {
   (T::from(1.0).unwrap() - x) * a + x * b
}

