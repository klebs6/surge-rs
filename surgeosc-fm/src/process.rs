crate::ix!();

impl OscillatorProcess for FMOscillator {

    fn process_block(&mut self, cfg: OscillatorProcessBlockCfg)
    {
        let a1: f64 = self.pvalf(FMOscillatorParam::M1Amount).into();
        let a2: f64 = self.pvalf(FMOscillatorParam::M2Amount).into();
        let a3: f64 = self.pvalf(FMOscillatorParam::M3Amount).into();

        let f_ratio1: f64 = self.pvalf(FMOscillatorParam::M1Ratio).into();
        let f_ratio2: f64 = self.pvalf(FMOscillatorParam::M2Ratio).into();
        let f_freq3:  f64 = self.pvalf(FMOscillatorParam::M3Freq).into();
        let feedback: f64 = self.pvalf(FMOscillatorParam::Feedback).into();

        self.driftlfo = drift_noise(self.driftlfo2) * cfg.drift;

        let drifted_pitch: f64 = (cfg.pitch + self.driftlfo) as f64;
        let drifted_omega: f64 = self.tuner.pitch2omega(drifted_pitch);

        let omega: f64 = mind(
            PI, 
            self.tuner.pitch2omega(drifted_pitch)
        );

        self.rm1.set_rate(
            mind(
                PI, 
                drifted_omega * f_ratio1
            )
        );

        self.rm2.set_rate(
            mind(
                PI, 
                drifted_omega * f_ratio2
            )
        );

        self.am.set_rate(
            mind(
                PI, 
                self.tuner.pitch2omega(60.0_f64 + f_freq3)
            )
        );

        self.rel_mod_depth1.new_value(32.0 * PI * a1 * a1 * a1);
        self.rel_mod_depth2.new_value(32.0 * PI * a2 * a2 * a2);
        self.abs_mod_depth.new_value( 32.0 * PI * a3 * a3 * a3);

        let depth = cfg.fm_depth as f64;

        if cfg.fm {
            self.fm_depth.new_value(32.0 * PI * depth * depth * depth);
        }

        self.feedback_depth.new_value(feedback);

        for k in 0..BLOCK_SIZE_OS {
            self.do_fm_block(k, omega, cfg.fm);
        }

        if cfg.stereo {
            self.out.dup_channel_to_stereo(StereoChannel::Left);
        }
    }
}
