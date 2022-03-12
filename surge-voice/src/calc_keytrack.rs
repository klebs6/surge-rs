ix!();

use crate::{
    SurgeVoice,
    VoiceRuntimeHandle,
};

impl SurgeVoice {

    pub fn calc_keytrack(&mut self,
        cfg: VoiceRuntimeHandle
    ) {
        let cfg = cfg.borrow();

        let keytrack_root = cfg.keytrack_root;

        let keytrack_f = (self.state.pitch - keytrack_root) * (ONE_TWELFTH as f64);

        if let Some(Box::new(ModulationSource::ControllerModulationSource(ref mut ms))) = 
            &mut self.modsources[ModSource::KeyTrack] {
                // I didn't change this for octaveSize, I think rightly
                ms.set_output(keytrack_f);
        }
    }
}
