ix!();

use crate::{
    WTOscillator,
};

impl WTOscillator {

    pub fn get_magic(ipos: u32) -> (__m128, u32) {

        let m: u32 = ((ipos >> 16) & 0xff) * ((FIR_IPOL_N << 1) as u32);

        let lipolui16: u32 = ipos & 0xffff;

        let lipol128: __m128 = unsafe {
            let mut lipol128 = _mm_cvtsi32_ss(z128![], lipolui16 as i32);
            lipol128 = _mm_shuffle_ps(lipol128, lipol128, _MM_SHUFFLE(0, 0, 0, 0));
            lipol128
        };

        (lipol128, m)
    }

    pub fn convolute_mono(&mut self, g: f32, delay: u32, ipos: u32) {

        let (lipol128, m) = Self::get_magic(ipos);

        let g128: __m128 = unsafe {
            let mut g128 = _mm_load_ss(&g);
            g128 = _mm_shuffle_ps(g128, g128, _MM_SHUFFLE(0, 0, 0, 0));
            g128
        };

        for k in (0..FIR_IPOL_N).step_by(4) {

            let kidx = (
                self.blitter.bufpos + (k as i32) + (delay as i32)
            ) as usize;

            let midx1 = (m + (k as u32) + (FIR_IPOL_N as u32)) as usize;
            let midx2 = (m + (k as u32)) as usize;

            let obf: *mut f32 = &mut self.blitter.oscbuffer_l[kidx];

            unsafe {
                let so: __m128 = {
                    let mut so = _mm_load_ps(&self.tables.sinctable(midx1));
                    so = _mm_mul_ps(so, lipol128);
                    so
                };

                let st: __m128 = {
                    let mut st = _mm_load_ps(&self.tables.sinctable(midx2));
                    st = _mm_add_ps(st, so); 
                    st = _mm_mul_ps(st, g128);
                    st
                };

                let ob: __m128 = {
                    let mut ob = _mm_loadu_ps(obf);
                    ob = _mm_add_ps(ob, st);
                    ob
                };

                _mm_storeu_ps(obf, ob);
            }
        }
    }
}
