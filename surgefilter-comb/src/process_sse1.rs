ix!();

#[cfg(not(target_arch = "x86_64"))] 
impl FilterProcessQuad for CombFilter {

    pub fn process_quad(&self, 
        qfu: &mut QuadFilterUnitState, 
        input: __m128) -> __m128 
    {
        unsafe {

            // changing the constant requires updating the code below
            assert!(FIR_IPOL_M == 256); 

            let m256:  __m128   = _mm_set1_ps(256.0);
            let m0xff: __m128i  = _mm_set1_epi32(0xff);

            qfu.C[0] = _mm_add_ps(qfu.C[0], qfu.dC[0]);
            qfu.C[1] = _mm_add_ps(qfu.C[1], qfu.dC[1]);

            let a: __m128 = _mm_mul_ps(qfu.C[0], m256);
            let DBRead: __m128 = _mm_setzero_ps();

            for i in (0..4).step_by(1) 
            {
                if qfu.active[i] > 0 {

                    let aptr: *mut f32 = (a.as_mut_ptr() as *mut f32).offset(i);
                    let e:  i32 = _mm_cvtss_si32(_mm_load_ss(aptr));
                    let DT: i32 = e >> 8;
                    let SE: i32 = (0xff - (e & 0xff)) * ((FIR_IPOL_N as i32) << 1);

                    let RP: i32 = ((qfu.WP[i] - DT - FIR_OFFSET as i32) as usize) & (MAX_FB_COMB - 1);

                    // SINC interpolation (12 samples)
                    let a: __m128 = _mm_loadu_ps(&qfu.DB[i][RP]);
                    let b: __m128 = _mm_load_ps(TABLES.sinctable.as_mut_ptr().offset(SE));
                    let o: __m128 = _mm_mul_ps(a, b);

                    a = _mm_loadu_ps(&qfu.DB[i][RP + 4]);
                    b = _mm_load_ps(TABLES.sinctable.as_mut_ptr().offset(SE + 4));
                    o = _mm_add_ps(o, _mm_mul_ps(a, b));

                    a = _mm_loadu_ps(&qfu.DB[i][RP + 8]);
                    b = _mm_load_ps(TABLES.sinctable.as_mut_ptr().offset(SE + 8));
                    o = _mm_add_ps(o, _mm_mul_ps(a, b));

                    let fptr: *mut f32 = (DBRead.as_mut_ptr() as *mut f32).offset(i);
                    _mm_store_ss(fptr, sum_ps_to_ss(o));
                }
            }

            let d: __m128 = _mm_add_ps(input, _mm_mul_ps(DBRead, qfu.C[1]));
            d = softclip_ps(d);

            for i in (0..4).step_by(1) 
            {
                if qfu.active[i] > 0 {

                    // Write to delaybuffer (with "anti-wrapping")
                    let t: __m128 = _mm_load_ss((&d as *mut f32).offset(i));

                    _mm_store_ss(&qfu.DB[i][qfu.WP[i]], t);

                    if qfu.WP[i] < (FIR_IPOL_N as i32) {
                        _mm_store_ss(&qfu.DB[i][qfu.WP[i] as usize + MAX_FB_COMB], t);
                    }

                    // Increment write position
                    qfu.WP[i] = ((qfu.WP[i] + 1) as usize) & (MAX_FB_COMB - 1);
                }
            }

            return _mm_add_ps(_mm_mul_ps(qfu.C[3], DBRead), _mm_mul_ps(qfu.C[2], input));
        }
    }
}
