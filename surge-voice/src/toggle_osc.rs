crate::ix!();

impl SurgeVoice {

    pub fn spawn_osc(&self, oscty: OscillatorType) -> Result<MaybeVoiceOscillator,SurgeError> {

        let tables   = self.tables.clone();
        let synth_in = self.synth_in.clone();
        let tuner    = self.tuner.clone();
        let srunit   = self.srunit.clone();

        Ok(match oscty {
            OscillatorType::AudioInput           => Some(Box::new(AudioInputOscillator::new(tables,synth_in))),
            OscillatorType::SurgeSuperOscillator => Some(Box::new(SurgeSuperOscillator::new(tuner,tables,srunit))),
            OscillatorType::FM                   => Some(Box::new(FMOscillator::new(tuner))),
            OscillatorType::FM2                  => Some(Box::new(FM2Oscillator::new(tuner,srunit))),
            OscillatorType::SampleAndHold        => Some(Box::new(SampleAndHoldOscillator::new(tuner,tables,srunit)?)),
            OscillatorType::Sine                 => Some(Box::new(SineWaveOscillator::new(tuner))),
            OscillatorType::Wavetable            => Some(Box::new(WTOscillator::new(tuner,tables,srunit)?)),
            OscillatorType::Window               => Some(Box::new(WindowOscillator::new(tuner,tables,srunit))),
            _ => None,
        })
    }

    pub fn maybe_toggle_osc(&mut self, cfg: VoiceRuntimeHandle) -> Result<(),SurgeError>
    {
        let cfg = cfg.borrow();

        for i in 0_usize..N_OSCS {

            let oscty = cfg.oscillator_type[i];

            if self.osctype[i] != oscty {

                self.osc[i] = self.spawn_osc(oscty)?;

                let enable = self.osc_enable[i];

                if enable {
                    if let Some(ref mut x) = &mut self.osc[i] {
                        x.init()?;
                        x.set_pitch(self.state.pitch as f32, false);
                    }
                }
                self.osctype[i] = oscty;
            }
        }

        Ok(())
    }
}
