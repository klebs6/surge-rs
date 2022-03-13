ix!();

use crate::*;

impl SurgeVoice {

    pub fn calc_modsources(&mut self, cfg: VoiceRuntimeHandle) {

        let cfg = cfg.borrow();

        if let Some(ref mut x) = self.modsources[ModSource::PolyphonicAfterTouch] {
            unsafe {
                x.set_output((*self.state.voice_channel_state).pressure as f64);
            }
        }

        if let Some(ref mut x) = self.modsources[ModSource::Timbre] {
            unsafe {
                x.set_output((*self.state.voice_channel_state).timbre as f64);
            }
        }

        if let Some(ref mut x) = self.modsources[ModSource::AmpEg] {
            x.process_block();
        }

        if let Some(ref mut x) = self.modsources[ModSource::FilterEg] {
            x.process_block();
        }

        if let Some(Box::new(ModulationSource::AdsrEnvelope(ref mut envelope))) = &mut self.modsources[ModSource::AmpEg] {
            if envelope.is_idle() {
                self.state.keep_playing = false;
            }
        }

        for item in cfg.modulation_voice.iter() {

            let src = item.src;
            let _dst = &item.dst;

            let _depth:  f32 = item.depth as f32;

            let _cur_src: Option<f32> = {
                match self.modsources[src].as_ref() {
                    Some(ref src) => {
                        match src {
                            Box::new(ModulationSource::ControllerModulationSource(_x)) => {
                                Some(src.get_output() as f32)
                            },
                            _ => None,
                        }
                    }
                    _ => None,
                }
            };

            todo!();
            /*
            if let Some(cur_src) = cur_src {
                if let Some(ref mut dst) = self.modsources[dst_id].as_mut() {
                    let cur_dst = dst.get_output() as f32;
                    let val = cur_dst + (depth * cur_src);
                    dst.set_output(val as f64);
                }
            }
            */
        }
    }
}
