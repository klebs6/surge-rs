ix!();

use crate::{
    AllpassReverbParam,
};

impl crate::AllpassVerb<'sr> {

    pub fn update_rtime<const N: usize>(&mut self) {

        let decay_time = pvalf![self.params[AllpassReverbParam::DecayTime]];
        let pre_delay  = pvalf![self.params[AllpassReverbParam::PreDelay]];

        let t: f32 = 
            block_size_inv![N] * ( 
                self.srunit.samplerate() *
                ( 1.0_f32.max(2.0_f32.powf(decay_time)) * 2.0 +
                  0.1_f32.max(2.0_f32.powf(pre_delay)) * 2.0
                ) 
            ); // *2 is to get the db120 time

        self.ringout = Ringout::blocks(t as NumberOfBlocks);
    }
}

impl Update for crate::AllpassVerb<'sr> {
    fn update(&mut self) {
        // TODO, balance the gains from the 
        // calculated decay coefficient?
        self.tap_gain_l[0] = 1.5 / 4.0;
        self.tap_gain_l[1] = 1.2 / 4.0;
        self.tap_gain_l[2] = 1.0 / 4.0;
        self.tap_gain_l[3] = 0.8 / 4.0;
        self.tap_gain_r[0] = 1.5 / 4.0;
        self.tap_gain_r[1] = 1.2 / 4.0;
        self.tap_gain_r[2] = 1.0 / 4.0;
        self.tap_gain_r[3] = 0.8 / 4.0;
        self.calc_size(1.0);
    }
}

