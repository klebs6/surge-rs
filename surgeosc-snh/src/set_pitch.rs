ix!();

use crate::{
    SampleAndHoldOscillator,
    SampleAndHoldOscillatorParam,
};

impl SetPitch for SampleAndHoldOscillator {

    fn set_pitch(&mut self, pitch: f32, is_display: bool) {

        if is_display {
            self.blitter.n_unison = 1;
            //srand(2);
        }

        self.blitter.prepare_unison(self.blitter.n_unison as usize);

        self.pitch = pitch;

        self.update_lagvals::<true>();

        for i in (0_usize..(self.blitter.n_unison as usize)).step_by(1) {

            if pvalb![self.osc_params[OscillatorParam::Retrigger]] || is_display {
                self.blitter.oscstate[i] = 0.0;
                self.blitter.syncstate[i] = 0.0;

            } else {

                let drand: f64 = rand01() as f64;

                let detune: f64 = 
                    (self.pvalf_extended(
                            SampleAndHoldOscillatorParam::UniSpread) as f64) *
                    (self.blitter.detune_bias * (i as f32) + 
                     self.blitter.detune_offset) as f64;

                let st: f64 = drand * 
                    self.tuner.n2p_tuningctr(detune) *
                    0.5;

                //value assigned is never read
                /*
                drand = rand01() as f64;

                let ot: f64 = drand * 
                    self.tuner.n2p_tuningctr(detune);
                */

                self.blitter.oscstate[i]  = st as f32;
                self.blitter.syncstate[i] = st as f32;
            }

            self.blitter.state[i]     = 0;
            self.last_level[i]        = 0.0;

            self.pwidth[i] = limit_range(
                self.l_pw.v as f32, 
                0.001, 
                0.999
            );

            self.blitter.driftlfo2[i] = 0.0;
            self.blitter.driftlfo[i]  = 0.0;
        }
    }
}
