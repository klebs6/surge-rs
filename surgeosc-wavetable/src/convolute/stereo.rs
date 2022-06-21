crate::ix!();

impl WTOscillator {

    pub fn convolute_stereo(&mut self, g: f32, delay: u32, ipos: u32) {

        let (lipol128, m) = Self::get_magic(ipos);

        let g128_l: __m128 = unsafe { 
            let mut g128_l = _mm_load_ss(&g);
            g128_l     = _mm_shuffle_ps(g128_l, g128_l, _MM_SHUFFLE(0, 0, 0, 0));
            g128_l
        };

        let g128_r: __m128 = unsafe { 
            let mut g128_r = _mm_load_ss(&g);
            g128_r     = _mm_shuffle_ps(g128_r, g128_r, _MM_SHUFFLE(0, 0, 0, 0));
            g128_r
        };

        for k in (0..FIR_IPOL_N).step_by(4) 
        {
            let kidx = (
                self.blitter.bufpos + (k as i32) + (delay as i32)
            ) as usize;

            let midx1 = (m + (k as u32) + (FIR_IPOL_N as u32)) as usize;
            let midx2 = (m + (k as u32)) as usize;

            let obf_l: *mut f32 = &mut self.blitter.oscbuffer_l[kidx];
            let obf_r: *mut f32 = &mut self.blitter.oscbuffer_r[kidx];

            unsafe {
                let so: __m128 = {
                    let mut so = _mm_load_ps(&self.tables.sinctable(midx1));
                    so = _mm_mul_ps(so, lipol128);
                    so
                };

                let st: __m128 = {
                    let mut st = _mm_load_ps(&self.tables.sinctable(midx2));
                    st = _mm_add_ps(st, so);
                    st
                };

                let ob_l: __m128 = {
                    let mut ob_l = _mm_loadu_ps(obf_l);
                    ob_l = _mm_add_ps(ob_l, _mm_mul_ps(st, g128_l));
                    ob_l
                };

                let ob_r: __m128 = {
                    let mut ob_r = _mm_loadu_ps(obf_r);
                    ob_r = _mm_add_ps(ob_r, _mm_mul_ps(st, g128_r));
                    ob_r
                };

                _mm_storeu_ps(obf_l, ob_l);
                _mm_storeu_ps(obf_r, ob_r);
            }
        }
    }
}
