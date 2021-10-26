ix!();

use crate::CombFilter;

#[cfg(target_arch = "x86_64")] 
impl FilterProcessQuad for CombFilter {

    #[allow(clippy::assertions_on_constants)]
    fn process_quad(&self, 
        qfu: &mut QuadFilterUnitState<'tables>, 
        input: __m128) -> __m128 
    {
        unsafe {
            // changing the constant requires updating the code below
            assert!(FIR_IPOL_M == 256); 

            let m256: __m128 = _mm_set1_ps(256.0);
            let m0xff: __m128i = _mm_set1_epi32(0xff);

            qfu.coeff[0] = _mm_add_ps(qfu.coeff[0], qfu.dcoeff[0]);
            qfu.coeff[1] = _mm_add_ps(qfu.coeff[1], qfu.dcoeff[1]);

            let a:  __m128 = _mm_mul_ps(qfu.coeff[0], m256);
            let e: __m128i = _mm_cvtps_epi32(a);

            let mut dti: [i32; 4] = Default::default();
            let mut sei: [i32; 4] = Default::default();

            let dt: __m128i = _mm_srli_epi32(e, 8);
            _mm_store_si128(dti.as_mut_ptr() as *mut __m128i, dt);

            let mut se: __m128i = _mm_and_si128(e, m0xff);
            se = _mm_sub_epi32(m0xff, se);

            _mm_store_si128(sei.as_mut_ptr() as *mut __m128i, se);

            let mut delay_buffer_read: __m128 = _mm_setzero_ps();

            for i in (0..4).step_by(1) 
            {
                if qfu.active[i] > 0 {

                    let read_position: usize = ((qfu.comb_write_position[i] - dti[i] - FIR_OFFSET as i32) as usize) & (MAX_FB_COMB - 1);

                    let ptr: *mut __m128i = sei.as_mut_ptr() as *mut __m128i;
                    let idx: isize = simd_extract::<__m128i, i64>(*ptr,i as u32) as isize;

                    // SINC interpolation (12 samples)
                    let mut a: __m128 = _mm_loadu_ps(qfu.delay_buffer[i].add(read_position));
                    sei[i] *= (FIR_IPOL_N as i32) << 1;
                    let mut b: __m128 = _mm_load_ps(qfu.tables.sinctable_ptr(idx));
                    let mut o: __m128 = _mm_mul_ps(a, b);

                    a = _mm_loadu_ps(qfu.delay_buffer[i].add(read_position + 4));
                    b = _mm_load_ps(qfu.tables.sinctable_ptr(idx + 4));
                    o = _mm_add_ps(o, _mm_mul_ps(a, b));

                    a = _mm_loadu_ps(qfu.delay_buffer[i].add(read_position + 8));
                    b = _mm_load_ps(qfu.tables.sinctable_ptr(idx as isize + 8));
                    o = _mm_add_ps(o, _mm_mul_ps(a, b));

                    let fptr: *mut f32 = ((&mut delay_buffer_read as *mut __m128) as *mut f32).add(i);

                    _mm_store_ss(fptr, sum_ps_to_ss(o));
                }
            }

            let mut d: __m128 = _mm_add_ps(input, _mm_mul_ps(delay_buffer_read, qfu.coeff[1]));
            d = softclip_ps(d);

            for i in (0..4).step_by(1) 
            {
                if qfu.active[i] > 0 {

                    // Write to delaybuffer (with "anti-wrapping")
                    let di_f32: f32 = simd_extract::<__m128, f32>(d,i as u32);

                    let t: __m128 = _mm_load_ss(&di_f32);

                    let write_position: isize = qfu.comb_write_position[i] as isize;

                    _mm_store_ss(qfu.delay_buffer[i].offset(write_position), t);

                    if write_position < (FIR_IPOL_N as isize) {
                        _mm_store_ss(qfu.delay_buffer[i].offset(write_position + MAX_FB_COMB as isize), t);
                    }

                    // Increment write position
                    qfu.comb_write_position[i] = ((write_position + 1) & (MAX_FB_COMB as isize - 1)) as i32;
                }
            }

            _mm_add_ps(
                _mm_mul_ps(qfu.coeff[3], delay_buffer_read), 
                _mm_mul_ps(qfu.coeff[2], input))
        }
    }
}
