ix!();

use crate::{
    FM2Oscillator,
    FM2OscillatorParam,
    fm2_calcmd,
};

impl OscillatorProcess for FM2Oscillator<'sr> {

    fn process_block(&mut self, cfg: OscillatorProcessBlockCfg) { 

        self.driftlfo = drift_noise(self.driftlfo2) * cfg.drift;

        let drifted_pitch:   f64 = (cfg.pitch + self.driftlfo) as f64;
        let omega:           f64 = self.tuner.pitch2omega(drifted_pitch) as f64;
        let omega_clamped:   f64 = mind(PI, omega);
        let i_ratio1:        i32 = self.pvali(FM2OscillatorParam::M1Ratio);
        let i_ratio2:        i32 = self.pvali(FM2OscillatorParam::M2Ratio);
        let a1:              f64 = self.pvalf(FM2OscillatorParam::M1Amount).into();
        let a2:              f64 = self.pvalf(FM2OscillatorParam::M2Amount).into();
        let shift:           f64 = (self.pvalf(FM2OscillatorParam::MxShift) as f64) * self.srunit.dsamplerate_inv();
        let startphase:      f64 = self.pvalf(FM2OscillatorParam::MxStartPhase).into();
        let feedback:        f64 = self.pvalf(FM2OscillatorParam::Feedback).into();
        let depth                = cfg.fm_depth as f64;

        self.rm1.set_rate(
            mind( PI, omega * (i_ratio1 as f64) + shift)
        );

        self.rm2.set_rate(
            mind( PI, omega * (i_ratio2 as f64) - shift)
        );

        self.rel_mod_depth1.new_value(fm2_calcmd(a1));
        self.rel_mod_depth2.new_value(fm2_calcmd(a2));

        if cfg.fm {
            self.fm_depth.new_value(32.0_f64 * PI * depth * depth * depth);
        }

        self.feedback_depth.new_value(feedback);

        self.phase_offset.new_value(2.0_f64 * PI * startphase);

        for k in 0..BLOCK_SIZE_OS {
            self.do_fm2_block(k, omega_clamped, cfg.fm);
        }

        if cfg.stereo
        {
            self.out.dup_channel_to_stereo(StereoChannel::Left);
        }
    }
}
