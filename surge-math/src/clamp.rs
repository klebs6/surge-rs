ix!();

use crate::*;

#[inline] pub fn clamp01<T: LimitRange + From<f32>> ( x: T) -> T 
{
    x.limit_range(T::from(0.0),T::from(1.0))
}

#[inline] pub fn clamp1_bipolar<T: LimitRange + From<f32>> ( x: T) -> T 
{
    x.limit_range(T::from(-1.0),T::from(1.0))
}

pub fn within_range<T: PartialOrd>(
    low: T, 
    val: T, 
    high: T) -> bool 
{
    (val >= low) && (val <= high)
}
