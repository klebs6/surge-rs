crate::ix!();

pub trait MyFloat = 
num_traits::Float 
+ std::ops::AddAssign 
+ Into<f64>;

pub type A3d<T> = ndarray::Array3::<T>;
pub type A2d<T> = ndarray::Array2::<T>;
pub type A1d<T> = ndarray::Array1::<T>;

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn max_ps_to_ss(x: __m128) -> __m128
{
    unsafe {
        let a: __m128 =  _mm_max_ss(x, _mm_shuffle_ps(x, x, _MM_SHUFFLE(0, 0, 0, 1)));
        let b: __m128 = 
            _mm_max_ss(
                _mm_shuffle_ps(x, x, _MM_SHUFFLE(0, 0, 0, 2)), 
                _mm_shuffle_ps(x, x, _MM_SHUFFLE(0, 0, 0, 3)));
        _mm_max_ss(a, b)
    }
}

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn sum_ps_to_ss(x: __m128) -> __m128 {
    unsafe {
        let a: __m128 = _mm_add_ps(x, _mm_movehl_ps(x, x));
        _mm_add_ss(a, _mm_shuffle_ps(a, a, _MM_SHUFFLE(0, 0, 0, 1)))
    }
}


//note, vt_read_* were macros in the c code
#[inline] pub fn vt_read_int32_le(t: i32) -> i32 {
    vt_write_int32_le(t)
}

#[inline] pub fn vt_read_int32_be(t: u32) -> u32 {
    vt_write_int32_be(t)
}

#[inline] pub fn vt_read_int16_le(t: i16) -> i16 {
    vt_write_int16_le(t)
}

#[inline] pub fn vt_read_int16_be(t: u16) -> u16 {
    vt_write_int16_be(t)
}

#[inline] pub fn vt_read_float32_le(t: f32) -> f32 {
    vt_write_float32_le(t)
}

/*
unsigned int Float2UInt(float x);
float sine_ss(unsigned int x);
*/

#[inline] pub fn vt_write_int32_le(t: i32) -> i32 { t }

#[inline] pub fn vt_write_float32_le(f: f32) -> f32 { f }

#[inline] pub fn vt_write_int32_be(t: u32) -> u32 
{
    // this was `swap_endian`:
    ((t << 24) & 0xff000000) | ((t << 8) & 0x00ff0000) | ((t >> 8) & 0x0000ff00) |
        ((t >> 24) & 0x000000ff)
}

#[inline] pub fn vt_write_int16_le(t: i16) -> i16 { t }

#[inline] pub fn vt_write_int16_be(t: u16) -> u16 
{
   ((t << 8) & 0xff00) | ((t >> 8) & 0x00ff)
}

#[inline] pub fn vt_copyblock_w_le(dst: *mut i16, src: *const i16, count: usize) 
{
    unsafe {
        libc::memcpy(dst as *mut c_void, src as *const c_void, count * std::mem::size_of::<i16>());
    }
}

#[inline] pub fn vt_copyblock_dw_le(dst: *mut i32, src: *const i32, count: usize) 
{
    unsafe {
        libc::memcpy(dst as *mut c_void, src as *const c_void, count * std::mem::size_of::<i32>());
    }
}

#[inline] pub fn v_load1(f: f32) -> __m128 
{
    unsafe {
        _mm_load1_ps(&f)
    }
}

#[macro_export] macro_rules! db60 {
    () => {{
        let db60: f64 = 10.0_f64.powf(0.05 * -60.0);
        db60
    }}
}

#[macro_export] macro_rules! db96 {
    () => {{
        let db96: f64 = 10.0_f64.powf(0.05 * -96.0);
        db96
    }}
}
