ix!();

use crate::*;

impl SurgeVoice {

    pub fn get_envelope_retrigger_gates(&self) -> (bool, bool) {

        let mut retrigger_aeg: bool = false;
        let mut retrigger_feg: bool = false;

        if let Some(box ModulationSource::Lfo(s)) 
            = &self.modsources[ModSource::VoiceLfo1] 
        {
            if s.retrigger_aeg {
                retrigger_aeg = true;
            }
            if s.retrigger_feg {
                retrigger_feg = true;
            }
        };

        (retrigger_aeg, retrigger_feg)
    }

    pub fn calc_envelopes(&mut self) {

        let (retrigger_aeg, retrigger_feg) = self.get_envelope_retrigger_gates();

        if retrigger_aeg {
            if let Some(box ModulationSource::AdsrEnvelope(ref mut adsr)) = 
                &mut self.modsources[ModSource::AmpEg] {
                    adsr.retrigger();
            }
        }

        if retrigger_feg {
            if let Some(box ModulationSource::AdsrEnvelope(ref mut adsr)) = 
                &mut self.modsources[ModSource::FilterEg] {
                    adsr.retrigger();
            }
        }
    }
}
