ix!();

use crate::{
    SurgeVoice,
    MaybeVoiceOscillator,
    VoiceRuntimeHandle,
};

impl SurgeVoice {

    pub fn spawn_osc(&self, oscty: OscillatorType) -> MaybeVoiceOscillator {

        let tables   = self.tables.clone();
        let synth_in = self.synth_in.clone();
        let tuner    = self.tuner.clone();
        let srunit   = self.srunit.clone();

        match oscty {
            OscillatorType::AudioInput           => Some(box AudioInputOscillator::new(tables,synth_in)),
            OscillatorType::SurgeSuperOscillator => Some(box SurgeSuperOscillator::new(tuner,tables,srunit)),
            OscillatorType::FM                   => Some(box FMOscillator::new(tuner)),
            OscillatorType::FM2                  => Some(box FM2Oscillator::new(tuner,srunit)),
            OscillatorType::SampleAndHold        => Some(box SampleAndHoldOscillator::new(tuner,tables,srunit)),
            OscillatorType::Sine                 => Some(box SineWaveOscillator::new(tuner)),
            OscillatorType::Wavetable            => Some(box WTOscillator::new(tuner,tables,srunit)),
            OscillatorType::Window               => Some(box WindowOscillator::new(tuner,tables,srunit)),
            _ => None,
        }
    }

    pub fn maybe_toggle_osc(&mut self, 
        cfg: VoiceRuntimeHandle) 
    {
        let cfg = cfg.borrow();

        for i in 0_usize..N_OSCS {

            let oscty = cfg.oscillator_type[i];

            if self.osctype[i] != oscty {

                self.osc[i] = self.spawn_osc(oscty);

                let enable = self.osc_enable[i];

                if enable {
                    if let Some(ref mut x) = &mut self.osc[i] {
                        x.init();
                        x.set_pitch(self.state.pitch as f32, false);
                    }
                }
                self.osctype[i] = oscty;
            }
        }
    }
}
