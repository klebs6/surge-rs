ix!();

use crate::{
    Chorus,
    ChorusParam,
    CHORUS_DEPTH,
};

impl<'sr> Chorus<'sr> {

    pub fn initial_update(&mut self) {

        let fb      = pvalf![self.params[ChorusParam::Feedback]];
        let mix     = pvalf![self.params[ChorusParam::Mix]];
        let highcut = pvalf![self.params[ChorusParam::HighCut]];
        let lowcut  = pvalf![self.params[ChorusParam::LowCut]];
        let width   = pvalf![self.params[ChorusParam::Width]];

        self.feedback.set_target(0.5 * amp_to_linear(fb));

        self.hp.coeff_hp(
            self.hp.calc_omega( self.pvalf(ChorusParam::LowCut) as f64 / 12.0), 
            0.707
        );

        self.hp.coeff_hp(self.hp.calc_omega(
                lowcut as f64 / 12.0), 0.707);

        self.lp.coeff_lp2b(self.lp.calc_omega( 
                highcut as f64  / 12.0), 0.707);

        self.mix.set_target( mix );

        self.width.set_target(self.tables.db_to_linear(width));
    }
}

impl<'sr> Update for Chorus<'sr> {

    fn update(&mut self) {

        let fb     = pvalf![self.params[ChorusParam::Feedback]];
        let rate   = pvalf![self.params[ChorusParam::Rate]];
        let time   = pvalf![self.params[ChorusParam::Time]];
        let mix    = pvalf![self.params[ChorusParam::Mix]];
        let width  = pvalf![self.params[ChorusParam::Width]];

        self.feedback.set_target_smoothed(0.5 * amp_to_linear(fb));

        let rate: f32 = self.tables.envelope_rate_linear(-rate) * 
            self.maybe_temposyncratio(ChorusParam::Time);

        let tm: f32 = { 
            self.tuner.n2p::<true,f32>(12.0 * time) * 
                self.maybe_temposyncratio_inv(ChorusParam::Time)
        };

        for i in 0..CHORUS_DEPTH {

            self.lfophase[i] += rate as f64;

            if self.lfophase[i] > 1.0 {
                self.lfophase[i] -= 1.0;
            }

            let lfoout: f32 = ((2.0 * (2.0 * self.lfophase[i] - 1.0).abs() - 1.0) as f32) * 
                self.pvalf(ChorusParam::Depth);

            self.time[i].new_value( 
                self.srunit.samplerate() * tm * (1.0 + lfoout)
            );
        }

        self.hp.coeff_hp(self.hp.calc_omega(
                self.pvalf(ChorusParam::LowCut) as f64 * 
                (1.0 / 12.0)), 0.707);

        self.lp.coeff_lp2b(self.lp.calc_omega(
                self.pvalf(ChorusParam::HighCut) as f64 * 
                (1.0 / 12.0)), 0.707);

        self.mix.set_target_smoothed(mix);

        self.width.set_target_smoothed(self.tables.db_to_linear(width));
    }
}

