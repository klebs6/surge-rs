ix!();

use crate::{
    WTOscillator,
};

impl OscillatorProcess for WTOscillator<'sr> {

    fn process_block(&mut self, cfg: OscillatorProcessBlockCfg) { 

        let stereo = cfg.stereo;
        let drift  = cfg.drift;
        let depth  = cfg.fm_depth;
        let fm     = cfg.fm;
        let pitch0 = cfg.pitch;

        self.pitch_last = self.pitch_t;
        self.pitch_t = minf(148.0, pitch0);

        self.blitter.pitchmult_inv =
            maxd(1.0, 
                self.srunit.dsamplerate_os() * 
                (1.0 / 8.175798915) * 
                self.tuner.n2pinv::<f64,false>(self.pitch_t as f64)
            ) as f32;

        // This must be a real division, reciprocal-approximation is not
        self.blitter.pitchmult = 1.0 / self.blitter.pitchmult_inv; 

        // precise enough
        self.drift = drift;

        self.process_lag();

        self.update_tables();

        match fm {
            true  => self.process_block_fm(depth,stereo),
            false => self.process_block_nofm(stereo),
        }

        let mut hpfblock = WetBlock1::<BLOCK_SIZE_OS>::default();

        unsafe {
            self.li_hpf.store_block(hpfblock.buf.as_mut_ptr(), BLOCK_SIZE_OS_QUAD);
        }

        for k in (0..BLOCK_SIZE_OS).step_by(1) {
            self.do_block(k, stereo, &mut hpfblock);
        }

        unsafe {
            clear_block::<BLOCK_SIZE_OS_QUAD>(
                &mut self.blitter.oscbuffer_l[self.blitter.bufpos as usize]
            );

            if stereo {
                clear_block::<BLOCK_SIZE_OS_QUAD>(
                    &mut self.blitter.oscbuffer_r[self.blitter.bufpos as usize]
                );
            }
        }

        self.blitter.bufpos = 
            (self.blitter.bufpos + BLOCK_SIZE_OS as i32) & ((OB_LENGTH - 1) as i32);

        self.maybe_handle_overlap(stereo);
    }
}
