ix!();

use crate::{
    Vocoder,
    VocoderBlockCfg,
    N_VOCODER_BANDS,
};

impl Vocoder {

    pub fn do_vocoder_block<const N: usize>(&mut self, 
        k:            usize, 
        modulator_in: &mut WetBlock1::<BLOCK_SIZE>,
        cfg:          &VocoderBlockCfg,
        data_l:       &mut [f32; N],
        data_r:       &mut [f32; N]) 
    {
        let input:       __m128 = v_load1(modulator_in.buf[k]);

        let left:     __m128 = v_load1(data_l[k]);
        let right:    __m128 = v_load1(data_r[k]);

        let mut left_sum:  __m128 = unsafe{ z128![] };
        let mut right_sum: __m128 = unsafe{ z128![] };

        assert!(self.active_bands <= N_VOCODER_BANDS as i32);

        for j in 0..(self.active_bands >> 2) {
            let ju = j as usize;

            let mut modulator: __m128 = self.modulator[ju].calc_bpf(input);

            unsafe {
                modulator = _mm_min_ps(_mm_mul_ps(modulator, modulator), cfg.max_level);

                modulator = _mm_and_ps(modulator, _mm_cmpge_ps(modulator, cfg.gate_level));

                self.env_f[ju] = v_madd(
                    self.env_f[ju], 
                    cfg.rate_m1, 
                    _mm_mul_ps(cfg.rate, modulator)
                );

                modulator = v_sqrt_fast(self.env_f[ju]);

                left_sum = _mm_add_ps(
                    left_sum, 
                    self.carrier_l[ju].calc_bpf(_mm_mul_ps(left, modulator))
                );

                right_sum = _mm_add_ps(
                    right_sum, 
                    self.carrier_r[ju].calc_bpf(_mm_mul_ps(right, modulator))
                );
            }
        }

        data_l[k] = v_sum(left_sum) * 4.0;
        data_r[k] = v_sum(right_sum) * 4.0;
    }
}
