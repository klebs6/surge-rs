crate::ix!();

pub trait WindowInput = 
std::cmp::PartialOrd 
+ std::ops::Mul<Output=f64> 
+ From<i32> 
+ std::ops::SubAssign 
+ Into<f64>;

pub fn blackman< T: WindowInput>(i: T, n: i32) 
    -> f64 
{
    let i: f64 = i.into();
    let n: f64 = n.into();
    0.42 - 
        0.5 * (2.0 * PI  * i / (n - 1.0)).cos() + 
        0.08 * (4.0 * PI * i / (n - 1.0)).cos()
}

pub fn symmetric_blackman< T: WindowInput >(i: T, n: i32) 
    -> f64 
{
    let mut i: f64 = i.into();
    let n: f64 = n.into();
    i -= n / 2.0;
    0.42 - 
        0.5 * (2.0 * PI  * i / (n)).cos() + 
        0.08 * (4.0 * PI * i / (n)).cos()
}

pub fn blackman_harris< T: WindowInput >(i: T, n: i32) 
    -> f64 
{
    let i: f64 = i.into();
    let n: f64 = n.into();
    0.35875 - 
        0.48829 * (2.0 * PI * i / (n - 1.0)).cos() + 
        0.14128 * (4.0 * PI * i / (n - 1.0)).cos() -
        0.01168 * (6.0 * PI * i / (n - 1.0)).cos()
}

pub fn symmetric_blackman_harris< T: WindowInput >(i: T, n: i32) 
    -> f64 
{
    let mut i: f64 = i.into();
    let n: f64 = n.into();
    i -= n / 2.0;
    0.35875 - 
        0.48829 * (2.0 * PI * i / (n)).cos() + 
        0.14128 * (4.0 * PI * i / (n - 1.0)).cos() -
        0.01168 * (6.0 * PI * i / (n)).cos()
}

pub fn hamming< T: WindowInput>(i: T, n: i32) 
    -> f64 
{
    let i: f64 = i.into();
    let n: f64 = n.into();

    if i >= n {
        return 0.0;
    }

    0.54 - 
        0.46 * (2.0 * PI * i / (n - 1.0)).cos()
}

pub fn hanning< T: WindowInput >( i: T, n: i32) 
    -> f64 
{
    let i: f64 = i.into();
    let n: f64 = n.into();

    if i >= n {
        return 0.0;
    }

    0.5 * 
        (1.0 - (2.0 * PI * i / (n - 1.0)).cos())
}

