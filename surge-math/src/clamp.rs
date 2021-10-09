ix!();

pub fn clamp01<T: std::cmp::PartialOrd + From<f32> > ( x: T) -> T 
{
    if x > 1.0.into() {
        return 1.0.into();
    }
    if x < 0.0.into() {
        return 0.0.into();
    }
    x
}

pub fn clamp1bp<T: std::cmp::PartialOrd + From<f32> > ( x: T) -> T 
{
    if x > 1.0.into() {
        return 1.0.into();
    }
    if x < (-1.0).into() {
        return (-1.0).into();
    }
    x
}

pub fn within_range<T: std::cmp::PartialOrd>(
    low: T, 
    val: T, 
    high: T) -> bool 
{
    (val >= low) && (val <= high)
}


#[cfg(target_arch = "x86_64")] #[inline] 
pub fn limit_range(x: f32, low: f32, high: f32) -> f32 {

    let mut result: f32 = 0.0;

    unsafe {_mm_store_ss(&mut result,
        _mm_min_ss(_mm_max_ss(_mm_load_ss(&x), _mm_load_ss(&low)), _mm_load_ss(&high))) };

    result
}

pub fn limit_range_i(x: i32, l: i32, h: i32) -> i32 
{
    std::cmp::max(std::cmp::min(x,h), l)
}

pub fn limit_range_u(x: u32, l: u32, h: u32) -> u32 
{
    std::cmp::max(std::cmp::min(x,h), l)
}

pub fn limit_range_f_nosimd(x: f32, min: f32, max: f32) -> f32 {
    std::cmp::max(std::cmp::min(FloatOrd(x),FloatOrd(max)), FloatOrd(min)).0
}

pub fn limit_range_f(x: f32, min: f32, max: f32) -> f32 
{
    let mut result: f32 = 0.0;
    unsafe {
        _mm_store_ss(&mut result,
            _mm_min_ss(_mm_max_ss(_mm_load_ss(&x),_mm_load_ss(&min)),_mm_load_ss(&max))); 
        result
    }
}


pub fn limit_range_d( x:f64, min:f64, max:f64) -> f64 
{
    std::cmp::max(std::cmp::min(FloatOrd(x),FloatOrd(max)), FloatOrd(min)).0
}

