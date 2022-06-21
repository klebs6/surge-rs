crate::ix!();

impl Update for Phaser {

    fn update(&mut self) {

        let rate: f64 = 
            (self.tables.envelope_rate_linear(-self.pvalf(PhaserParam::LFORate)) as f64) *
            (self.maybe_temposyncratio(PhaserParam::LFORate) as f64);

        self.lfophase += (SLOWRATE as f32) * (rate as f32);

        if self.lfophase > 1.0 {
            self.lfophase -= 1.0;
        }

        let mut lfophase_r: f32 = self.lfophase + 
            0.5 * 
            self.pvalf(PhaserParam::Stereo);


        if lfophase_r > 1.0 {
            lfophase_r -= 1.0;
        }

        let lfoout: f64 = 1.0_f64 - ((2.0 - 4.0 * self.lfophase).abs() as f64);
        let lfoout_r: f64 = 1.0_f64 - ((2.0 - 4.0 * lfophase_r).abs() as f64);

        for i in 0_usize..(PHASER_N_BQ as usize)
        {
            let mut omega: f64 = self.biquad[0].calc_omega(2.0_f64 * 
                (self.pvalf(PhaserParam::Base) as f64) +
                (PHASER_BASEFREQ[i] as f64) +
                (PHASER_BASEPAN[i] as f64) * 
                (lfoout as f64) * 
                (self.pvalf(PhaserParam::LFODepth) as f64));

            let q = self.pvalf(PhaserParam::QualityFactor) as f64;
            self.biquad[i].coeff_apf(omega, 1.0_f64 + 0.8 * q);

            omega = self.biquad[0].calc_omega(2.0_f64 * 
                (self.pvalf(PhaserParam::Base) as f64) +
                (PHASER_BASEFREQ[i] as f64) +
                (PHASER_BASEPAN[i] as f64) * 
                (lfoout_r as f64) * 
                (self.pvalf(PhaserParam::LFODepth) as f64));

            self.biquad[i + PHASER_N_BQ as usize].coeff_apf(
                omega, (1.0 + 0.8 * q as f32) as f64
            );
        }

        self.feedback.new_value(0.95 * self.pvalf(PhaserParam::Feedback));
    }
}
