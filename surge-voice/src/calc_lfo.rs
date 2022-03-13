ix!();

use crate::*;

impl SurgeVoice {

    pub fn calc_lfos(&mut self, 
        cfg: VoiceRuntimeHandle) 
    {
        let cfg = cfg.borrow();

        assert!(cfg.update_lfo[0]);

        macro_rules! maybe_do_lfo {
            ($cfg:expr,$i:expr,$e:expr) => {{
                //TODO: can we sort out modsource_doprocess more cleanly?
                if let Some(ref mut lfo) = self.modsources[$e] {
                    if $cfg.update_lfo[$i] {
                        lfo.process_block();
                    }
                }
            }}
        }

        if let Some(ref mut lfo) = self.modsources[ModSource::VoiceLfo1] {
            // Always process LFO1 so the gate retrigger always work
            lfo.process_block();
        }

        maybe_do_lfo![cfg, 1, ModSource::VoiceLfo2];
        maybe_do_lfo![cfg, 2, ModSource::VoiceLfo3];
        maybe_do_lfo![cfg, 3, ModSource::VoiceLfo4];
        maybe_do_lfo![cfg, 4, ModSource::VoiceLfo5];
        maybe_do_lfo![cfg, 5, ModSource::VoiceLfo6];
    }
}
