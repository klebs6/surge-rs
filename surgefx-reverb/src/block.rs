ix!();

use crate::{
    Reverb,
    ReverbParam,
    REVERB_MAX_DELAY,
    REVERB_TAP_BITS,
    REVERB_TAPS,
};

struct ReverbBlockCfg {
    pub damp4:         __m128,
    pub predelay_time: i32,
}

impl Reverb {

    //TODO: fix this function
    #[inline] pub fn delay_idx(&self, tap: usize, offset: usize) -> usize {

        let mut dp: i32 = 
            self.delay_pos as i32 
            - ( self.delay_time[tap + offset] as i32 >> 8);

        dp &= REVERB_MAX_DELAY as i32 - 1;

        dp <<=  REVERB_TAP_BITS;

        assert!(dp >= 0);

        tap + offset + (dp as usize)
    }

    #[inline] pub fn calculate_predelay_time(&self) -> i32 {

        let sr:  f32 = (self.srunit.samplerate() as i32) as f32; //cast to truncate

        let predelay = self.pvalf(ReverbParam::PreDelay);

        let pitch    = self.tuner.n2p::<f32,true>(12.0 * predelay);

        let tsr_inv  = self.maybe_temposyncratio_inv(ReverbParam::PreDelay);

        ( sr * pitch * tsr_inv ) as i32
    }

    #[inline] pub fn process_filter_bands(&mut self, wet: &mut WetBlock) {

        unsafe {

            let wet_l = wet.l.as_mut_ptr();
            let wet_r = wet.r.as_mut_ptr();

            self.locut.process_block_slowlag(wet_l, wet_r);
            self.band1.process_block_slowlag(wet_l, wet_r);
            self.hicut.process_block_slowlag(wet_l, wet_r);
        }
    }

    #[inline] pub fn do_reverb<const N: usize>(&mut self, 
        wet:           &mut WetBlock,
        data_l:        &mut [f32; N],
        data_r:        &mut [f32; N])
    {
        let predelay_time = self.calculate_predelay_time();

        let damp4:   __m128 = unsafe { _mm_load1_ps(&self.pvalf(ReverbParam::Damping)) };

        for block_idx in 0_usize..BLOCK_SIZE {
            self.do_reverb_block(
                block_idx,
                ReverbBlockCfg {
                    damp4,
                    predelay_time, 
                },
                wet, 
                data_l, 
                data_r);
        }
    }

    pub fn do_pre_taps(&mut self, damp4: __m128) {

        let damp4m1 = unsafe {
            _mm_sub_ps(_mm_set1_ps(1.0), damp4)
        };

        for tap in (0..REVERB_TAPS).step_by(4) {
            self.do_pre_tap(tap,damp4,damp4m1);
        }
    }

    pub fn do_post_taps(&mut self, predelay_time: i32) -> (__m128, __m128) {

        let fb = self.get_fb(predelay_time);

        unsafe {

            let fb4: __m128 = _mm_shuffle_ps(fb, fb, 0);

            let mut l: __m128 = _mm_setzero_ps(); 
            let mut r: __m128 = _mm_setzero_ps(); 

            for tap in (0..REVERB_TAPS).step_by(4) {
                self.do_post_tap(tap, fb4, &mut l, &mut r);
            }

            (l, r)
        }
    }

    #[inline] fn do_reverb_block<const N: usize>(&mut self, 
        block_idx: usize,
        cfg:       ReverbBlockCfg,
        wet:       &mut WetBlock,
        data_l:    &mut [f32; N],
        data_r:    &mut [f32; N])
    {
        self.do_pre_taps(cfg.damp4);

        self.delay_pos = ( self.delay_pos + 1) & (REVERB_MAX_DELAY - 1);

        self.predelay[self.delay_pos] 
            = 0.5 * (
                data_l[block_idx] 
                + data_r[block_idx]
            );

        unsafe {

            let (mut l, mut r) = self.do_post_taps(cfg.predelay_time);

            l = sum_ps_to_ss(l);
            r = sum_ps_to_ss(r);

            _mm_store_ss(wet.l.as_mut_ptr().add(block_idx), l);
            _mm_store_ss(wet.r.as_mut_ptr().add(block_idx), r);
        }
    }
}
