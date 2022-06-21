crate::ix!();

impl SurgeVoice {

    pub fn maybe_calc_mpe(&mut self, 
        cfg: VoiceRuntimeHandle) 
    {
        let cfg = cfg.borrow();

        if self.mpe_enabled.0 {

            // See github issue 1214. This basically compensates for
            // channel AT being per-voice in MPE mode (since it is per channel)
            // vs per-scene (since it is per keyboard in non MPE mode).
            for item in cfg.modulation_scene.iter() {

                let src = item.src;

                if src == ModSource::ChannelAfterTouch && self.modsources[src].is_some() {

                    let _dst = &item.dst;

                    let _depth: f64 = item.depth;

                    todo!();
                    /*
                    let cur_dst: f64 = self.modsources[dst_id].as_ref().unwrap().get_output();
                    let cur_src: f64 = self.modsources[src].as_ref().unwrap().get_output();

                    self.modsources[dst_id].as_mut().unwrap().set_output( 
                        cur_dst + depth * cur_src );
                    */
                }
            }
        }
    }
}
