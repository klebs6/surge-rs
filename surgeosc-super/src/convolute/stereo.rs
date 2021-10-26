ix!();

use crate::SurgeSuperOscillator;

impl SurgeSuperOscillator {

    #[inline] pub fn convolve_stereo(&mut self, 
        g:        f32, 
        g_r:       f32, 
        delay:    usize, 
        m:        usize,
        lipol128: __m128) 
    {
        unsafe {

            let mut g128_l: __m128 = _mm_load_ss(&g);
            g128_l = _mm_shuffle_ps(g128_l, g128_l, _MM_SHUFFLE(0, 0, 0, 0));

            let mut g128_r: __m128 = _mm_load_ss(&g_r);
            g128_r = _mm_shuffle_ps(g128_r, g128_r, _MM_SHUFFLE(0, 0, 0, 0));

            for k in (0..FIR_IPOL_N).step_by(4) {

                let idx: usize = (self.blitter.bufpos as usize) + (k as usize) + (delay as usize);

                let sincidx:  usize = m as usize + k;
                let sincidx2: usize = m as usize + k + FIR_IPOL_N;

                let obf_l = &mut self.blitter.oscbuffer_l[idx];
                let obf_r = &mut self.blitter.oscbuffer_r[idx];

                let mut ob_l: __m128 = _mm_loadu_ps(obf_l);
                let mut ob_r: __m128 = _mm_loadu_ps(obf_r);
                let mut st:   __m128 = _mm_load_ps(self.tables.sinctable_ptr(sincidx));
                let mut so:   __m128 = _mm_load_ps(self.tables.sinctable_ptr(sincidx2));

                so   = _mm_mul_ps(so, lipol128);
                st   = _mm_add_ps(st, so);
                ob_l = _mm_add_ps(ob_l, _mm_mul_ps(st, g128_l));

                _mm_storeu_ps(obf_l, ob_l);

                ob_r = _mm_add_ps(ob_r, _mm_mul_ps(st, g128_r));

                _mm_storeu_ps(obf_r, ob_r);
            }
        }
    }
}
