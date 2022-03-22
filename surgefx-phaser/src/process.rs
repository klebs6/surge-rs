ix!();

use crate::*;

impl Process for Phaser {

    fn process<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N])
    {
        if self.bi == 0 {
            self.update();
        }

        self.bi = (self.bi + 1) & (SLOWRATE_M1 as i32);

        for k in 0..N
        {
            self.do_phaser_block(k,data_l,data_r);
        }

        let mix = pvalf![self.params[PhaserParam::Mix]];

        self.mix.set_target_smoothed(limit_range(mix, 0.0, 1.0));

        unsafe {
            self.mix.fade_2_blocks_to(
                data_l.as_mut_ptr(), 
                self.l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                self.r.as_mut_ptr(), 
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                block_size_quad![N]);
        }
    }
}

impl ProcessOnlyControl for Phaser {

    fn process_only_control<const N: usize>(&mut self) { 

        let rate: f32 = 
            self.tables.envelope_rate_linear(-self.pvalf(PhaserParam::LFORate)) *
            self.maybe_temposyncratio(PhaserParam::LFORate);

        self.lfophase += (SLOWRATE as f32) * rate;

        if self.lfophase > 1.0 {
            self.lfophase -= 1.0;
        }

        //the value assigned to lfophaseR is never read
        /*
        let mut lfophaseR: f32 = self.lfophase + 0.5 * self.pvalf(PhaserParam::Stereo);

        if lfophaseR > 1.0 {
            lfophaseR -= 1.0;
        }
        */
    }
}
