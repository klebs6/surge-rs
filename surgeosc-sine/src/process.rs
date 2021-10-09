ix!();

use crate::{
    SineWaveOscillator,
    SineWaveOscillatorParam,
};

impl OscillatorProcess for SineWaveOscillator<'sr> {

    fn process_block(&mut self, 
        cfg: OscillatorProcessBlockCfg)
    {
        let fm       = cfg.fm;
        let fm_depth = cfg.fm_depth;
        let stereo   = cfg.stereo;
        let drift    = cfg.drift;
        let pitch    = cfg.pitch;

        if self.pvali(
            SineWaveOscillatorParam::FMBehavior) == 0 
        {
            self.process_block_legacy(pitch, 
                drift, 
                stereo, 
                fm, 
                fm_depth 
            );
            return;
        }

        self.driftlfo1 = drift_noise(self.driftlfo2);

        let omega: f64 = mind(
            PI, 
            self.tuner.pitch2omega(
                (pitch + drift * self.driftlfo1) as f64
            )
        );

        self.fm_depth.new_value(32.0 * PI * 
            (fm_depth as f64) * (fm_depth as f64) * (fm_depth as f64));

        self.feedback.new_value(
            self.pvalf(SineWaveOscillatorParam::Feedback) as f64
        );

        for k in 0..BLOCK_SIZE_OS {
            self.do_sine_block(k, omega, fm);
        }

        if cfg.stereo
        {
            self.out.dup_channel_to_stereo(
                StereoChannel::Left);
        }
    }
}
