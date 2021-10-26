ix!();

use crate::{
    DualDelay,
    DualDelayParam,
};

impl Update for DualDelay {

    fn update(&mut self) {

        let feedback:  f32 = amp_to_linear(self.pvalf(DualDelayParam::Feedback));
        let crossfeed: f32 = amp_to_linear(self.pvalf(DualDelayParam::CrossFeed));

        self.feedback.set_target_smoothed(feedback);
        self.crossfeed.set_target_smoothed(crossfeed);

        let lforate: f64 = (
            self.tables.envelope_rate_linear(-self.pvalf(DualDelayParam::Rate)) * 
            self.maybe_temposyncratio(DualDelayParam::Rate)
        ).into();

        self.lfophase += lforate;

        if self.lfophase > 0.5 {
            self.lfophase -= 1.0;
            self.lfo_direction = !self.lfo_direction;
        }

        let lfo_increment: f32 = (
            0.00000000001 + 
            2.0_f32.powf(
                self.pvalf(DualDelayParam::Depth) * (1.0 / 12.0)
            ) - 1.0
        ) * (BLOCK_SIZE as f32);

        // small bias to avoid denormals

        let ca: f32 = 0.99;

        if self.lfo_direction {
            self.lfo_val = ca * self.lfo_val + lfo_increment;
        } else {
            self.lfo_val = ca * self.lfo_val - lfo_increment;
        }

        let l = self.pvalf(DualDelayParam::Left);
        let r = self.pvalf(DualDelayParam::Right);

        self.time_l.new_value(
            self.srunit.samplerate() 
            * (
                self.maybe_temposyncratio_inv(DualDelayParam::Left) *
                self.tuner.n2p::<f32,true>(12.0 * l)
            ) 
            + self.lfo_val - FIR_OFFSET_F32
        );

        self.time_r.new_value(
            self.srunit.samplerate()
            * (
                self.maybe_temposyncratio_inv(DualDelayParam::Right) *
                self.tuner.n2p::<f32,true>(12.0 * r)
            ) 
            + self.lfo_val - FIR_OFFSET_F32
        );

        let maxfeedback: f32 = maxf(db96![] as f32, feedback + crossfeed);

        if maxfeedback < 1.0 {

            let f: f32 = BLOCK_SIZE_INV * 
                maxf(self.time_l.v, self.time_r.v) * 
                ((1.0 + db96![].log(maxfeedback as f64)) as f32);

            self.ringout = Ringout::blocks(f as NumberOfBlocks);

        } else {

            self.ringout = Ringout::blocks(0);
        }

        let mix     = self.pvalf(DualDelayParam::Mix);
        let width   = self.pvalf(DualDelayParam::Width);
        let pan     = self.pvalf(DualDelayParam::Pan);
        let lowcut  = self.pvalf(DualDelayParam::LowCut);
        let highcut = self.pvalf(DualDelayParam::HighCut);

        self.mix.set_target_smoothed(mix);

        self.width.set_target_smoothed(self.tables.db_to_linear(width));

        self.pan.set_target_smoothed(limit_range(pan, -1.0, 1.0));

        let hp_omega = self.hp.calc_omega(lowcut as f64 / 12.0);
        let lp_omega = self.lp.calc_omega(highcut as f64 / 12.0);

        self.hp.coeff_hp(hp_omega, 0.707);
        self.lp.coeff_lp2b(lp_omega, 0.707);
    }
}

#[test] fn test_log() {
    let maxfeedback: f64 = 1323.434;
    let f1: f64 = 1.0 + db96![].log10() / maxfeedback.log10();
    let f2: f64 = 1.0 + db96![].log(maxfeedback);
    assert!(f1 == f2);
}

