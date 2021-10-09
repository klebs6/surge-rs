ix!();

#[cfg(target_arch = "x86_64")] #[inline] 
pub fn saturate(mut f: f32) -> f32
{
   let mut x: __m128 = unsafe{ _mm_load_ss(&f) };

   let x_min: __m128 = unsafe{ _mm_set_ss(-1.0) };
   let x_max: __m128 = unsafe{ _mm_set_ss(1.0)  };

   x = unsafe{ _mm_max_ss(_mm_min_ss(x, x_max), x_min) };

   unsafe{ _mm_store_ss(&mut f, x) };

   f
}

pub fn boundfreq(freq: &mut f32) {
    if *freq > 75.0 {
        *freq = 75.0;
    }
    if *freq < -55.0 {
        *freq = -55.0;
    }
}
