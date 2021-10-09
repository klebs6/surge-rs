
ix!();

use crate::{
    WTOscillator,
};

impl WTOscillator<'sr> {

    #[inline] pub fn do_block(&mut self, k: usize, stereo: bool, hpfblock: &mut WetBlock1::<BLOCK_SIZE_OS>) { 

        unsafe {
            let hpf: __m128 = _mm_load_ss(&hpfblock.buf[k]);
            let ob:  __m128 = _mm_load_ss(&self.blitter.oscbuffer_l[(self.blitter.bufpos as usize + k)]);
            let a:   __m128 = _mm_mul_ss(self.blitter.osc_out_l, hpf);

            self.blitter.osc_out_l = _mm_add_ss(a, ob);
            _mm_store_ss(&mut self.out.l[k], self.blitter.osc_out_l);

            if stereo {

                let ob: __m128 = _mm_load_ss(&self.blitter.oscbuffer_r[(self.blitter.bufpos as usize + k)]);
                let a: __m128 = _mm_mul_ss(self.blitter.osc_out_r, hpf);

                self.blitter.osc_out_r = _mm_add_ss(a, ob);
                _mm_store_ss(&mut self.out.r[k], self.blitter.osc_out_r);
            }
        }
    }
}
