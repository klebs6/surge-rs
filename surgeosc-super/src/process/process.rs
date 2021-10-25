ix!();

use crate::{
    SurgeSuperOscillator,
    SSOBlockCfg,
};

impl OscillatorProcess for SurgeSuperOscillator<'sr> {

    fn process_block(&mut self, cfg: OscillatorProcessBlockCfg) {
        let pitch0 = cfg.pitch;

        /* So lets tie these comments back to the description at the top. 
         * Start by setting up your time and wavelength based on the note */
       self.pitch = minf(148.0, pitch0);
       self.drift = cfg.drift;

       self.blitter.pitchmult_inv =
           maxd(
               1.0, 
               self.srunit.dsamplerate_os() * (1.0 / 8.175798915) * 
               self.tuner.n2pinv::<f64,false>(self.pitch as f64)
           ) as f32;

       // This must be a real division, reciprocal-approximation is not precise enough
       self.blitter.pitchmult =
           1.0 /
           self.blitter.pitchmult_inv; 

       /* And step all my internal parameters */
       self.update_lagvals(false);
       self.l_pw.process();
       self.l_pw2.process();
       self.l_shape.process();
       self.l_sub.process();
       self.l_sync.process();

       match cfg.fm {
           true  => self.process_block_fm(&cfg),
           false => self.process_block_nofm(&cfg),
       };

       /* OK so load up the hpf across the block 
          (linearly moving to target if target has changed) */
       let mut hpfblock = WetBlock1::<BLOCK_SIZE_OS>::default();

       unsafe {

           self.li_hpf.store_block(hpfblock.buf.as_mut_ptr(), 
               BLOCK_SIZE_OS_QUAD);

           /* And the DC offset and pitch-scaled 
              output attenuation */
           let mut mdc: __m128 = _mm_load_ss(&mut self.dc as *mut f32);
           let mut oa:  __m128 = _mm_load_ss(&mut self.blitter.out_attenuation as *mut f32);
           oa = _mm_mul_ss(oa, _mm_load_ss(&mut self.blitter.pitchmult as *mut f32));

           /* The Coef's here are from the character filter, 
              and are set in ::init */
           let char_a1: __m128 = _mm_load_ss(&mut self.coeff_a1 as *mut f32);
           let char_b0: __m128 = _mm_load_ss(&mut self.coeff_b0 as *mut f32);
           let char_b1: __m128 = _mm_load_ss(&mut self.coeff_b1 as *mut f32);

           for k in 0_usize..(BLOCK_SIZE_OS as usize) {
               println!("doing block");
               self.do_block(
                   SSOBlockCfg{
                       hpfblock: &hpfblock,
                       k,
                       stereo: cfg.stereo,
                       mdc: &mut mdc, 
                       oa: &mut oa, 
                       char_a1: &char_a1, 
                       char_b0: &char_b0, 
                       char_b1: &char_b1
                   }
               );
           }

           /* Store the DC accumulation */
           _mm_store_ss(&mut self.dc, mdc);

           let bufidx = self.blitter.bufpos as usize;

           /* And clean up and advance our buffer pointer */
           clear_block::<BLOCK_SIZE_OS_QUAD>(&mut self.blitter.oscbuffer_l[bufidx]);

           if cfg.stereo {
               clear_block::<BLOCK_SIZE_OS_QUAD>(&mut self.blitter.oscbuffer_r[bufidx]);
           }

           clear_block::<BLOCK_SIZE_OS_QUAD>(&mut self.blitter.dcbuffer[bufidx]);

           self.blitter.bufpos = ((bufidx + BLOCK_SIZE_OS) & (OB_LENGTH - 1)) as i32;

           self.maybe_handle_wrap(cfg.stereo);
       }

       self.first_run = false;
    }
}
