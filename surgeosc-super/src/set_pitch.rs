crate::ix!();

impl SetPitch for SurgeSuperOscillator {

    fn set_pitch(&mut self, pitch: f32, is_display: bool) {

        if is_display {
            self.blitter.n_unison = 1;
        }

        self.pitch = pitch;
        self.update_lagvals(true);

        let retrigger = pvalb![self.osc_params[OscillatorParam::Retrigger]];

        for i in 0_usize..(self.blitter.n_unison as usize) {

            if  retrigger || is_display {
                self.blitter.oscstate[i] = 0.0; 
                self.blitter.syncstate[i] = 0.0;
                self.last_level[i] = -0.4;

            } else {

                let spread = self.pvalf_extended(SSOParam::UniSpread);
                let drand:  f64 = rand01() as f64;
                let detune: f32 = spread * (self.blitter.detune_bias * (i as f32) + self.blitter.detune_offset);
                let st:     f64 = 0.5 * drand * self.tuner.n2pinv_tuningctr(detune as f64);

                self.blitter.oscstate[i] = st as f32;
                self.blitter.syncstate[i] = st as f32;
                self.last_level[i] = 0.0;
            }
            self.dc_uni[i] = 0.0;
            self.blitter.state[i] = 0;
            self.pwidth[i] = limit_range(self.l_pw.v, 0.001, 0.999);
            self.blitter.driftlfo2[i] = 0.0;
            self.blitter.driftlfo[i] = 0.0;
        }
    }
}
