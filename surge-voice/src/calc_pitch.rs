ix!();

use crate::{
    SurgeVoice,
    VoiceRuntimeHandle,
};

impl SurgeVoice<'sr> {

    #[inline] pub fn get_pitchbend(&self, cfg: &PitchBendCfg) -> f64 {

        let mut pb: f64 = self.modsources[
            ModSource::PitchBend
        ].as_ref().unwrap().get_output() as f64;

        pb *= match pb {
            _ if pb > 0.0 => cfg.range_up   as f64,
            _             => cfg.range_down as f64,
        };

        pb
    }

    #[inline] pub fn update_octave_size_and_return(&mut self) -> f64 {

        let is_standard_tuning: bool = self.tuner.current_tuning_is_standard();

        match is_standard_tuning {
            true  => {
                self.octave_size = SurgeTuner::default_scale_count();
            },
            false => {
                self.octave_size = self.tuner.current_scale_count();
            }
        }
        self.octave_size as f64
    }

    pub fn calc_pitch(&mut self, 
        cfg: VoiceRuntimeHandle<'sr>) 
    {
        let cfg = cfg.borrow();

        let pitch_id    = cfg.pitch;
        let octave_id   = cfg.octave;
        let pitchbend   = self.get_pitchbend(&cfg.pitchbend_cfg);
        let octave_size = self.update_octave_size_and_return();

        let maybe_extend_range = match cfg.pitch_extend_range {
            true  => 12.0_f64,
            false => 1.0_f64,
        };

        self.state.pitch = 
            self.state.pkey 
            + pitchbend 
            + pitch_id * maybe_extend_range
            + octave_size * octave_id;
    }
}
