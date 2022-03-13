ix!();

use crate::*;

impl SurgeVoice {

    pub fn modsource_attack(&mut self) {
        macro_rules! attack {
            ($t:ident, $x:ident) => {
                if let Some(box ModulationSource::$t(ref mut item)) 
                    = &mut self.modsources[ModSource::$x] 
                { 
                    item.attack(); 
                }
            }
        }
        attack![AdsrEnvelope, AmpEg];
        attack![AdsrEnvelope, FilterEg];
        attack![Lfo,          VoiceLfo1];
        attack![Lfo,          VoiceLfo2];
        attack![Lfo,          VoiceLfo3];
        attack![Lfo,          VoiceLfo4];
        attack![Lfo,          VoiceLfo5];
        attack![Lfo,          VoiceLfo6];
    }
}
