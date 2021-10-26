ix!();

use crate::{
    SurgeVoice,
    VoiceRuntimeHandle,
};

impl SurgeVoice {

    pub fn switch_toggled(&mut self, 
        cfg: VoiceRuntimeHandle) 
    {
        {
            let cfg = cfg.borrow();

            self.update_portamento(
                cfg.portamento, 
                cfg.portamento_temposync
            );

            let pb = self.get_pitchbend(&cfg.pitchbend_cfg);

            self.state.pitch = self.state.pkey + (pb as f64);

            let out = self.state.pitch - (cfg.keytrack_root as f64 * ONE_TWELFTH as f64);

            if let Some(ref mut modsource) = self.modsources[ModSource::KeyTrack] {
                modsource.set_output(out);
            }
        }

        self.maybe_toggle_osc(cfg.clone());
        self.maybe_toggle_fm(&cfg.borrow().fm_cfg);
        self.maybe_toggle_solo(cfg.clone());
        self.maybe_toggle_filter(cfg.clone());
    }
}
