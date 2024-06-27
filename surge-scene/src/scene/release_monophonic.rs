crate::ix!();

#[derive(Debug)]
pub struct PlayConfig {
    pub active_voice_channel:     i32,
    pub active_voice_key:         i32,
    pub velocity:                 i32,
    pub lastdetune:               u8,
}

impl SurgeScene {

    pub fn release_monophonic(
        &mut self, 
        voice_idx: usize,
        cfg: ReleaseCfg

    ) -> Result<(),SurgeError> {

        let scene_non_ultra_release_voices: i32 = 
            self.get_non_ultra_release_voices();

        let mut do_switch: bool = false;
        let mut play_configs = vec![];

        let polymode = {
            let idx = pvali![
                self.params[SceneParam::PolyMode]
            ];
            PolyMode::try_from(idx as usize).unwrap()
        };

        let voice = &mut self.voices[voice_idx];

        // input these modes, our job when we
        // release a note is to see if any ohter
        // note is held.
        // 
        // input normal midi mode, that means
        // scanning the keystate of our channel
        // looking for another note.
        // 
        // input MPE mode, where each note is per
        // channel, that means scanning all
        // non-main channels rather than ourself
        // for the highest note
        if voice.borrow().state.key == (cfg.key as i32) && 
            voice.borrow().state.channel == (cfg.channel as i32) 
        {

            //these will be overridden
            let mut active_voice_key = 60;
            let mut active_voice_channel = 0;

            voice.borrow_mut().release();

            let channel_range = 1_usize..16_usize;
            let lowkey = 0;

            match self.mpe_unit.enabled() {

                MpeEnableSwitch(true) => {
                    for k in cfg.keyrange.clone() 
                    {
                        if do_switch { break; }

                        for mpe_chan in channel_range.clone() {
                            if mpe_chan != (cfg.channel as usize) && 
                                (self.midi_unit.keystate(mpe_chan as u8, k as u8) != 0) 
                            {
                                do_switch = true;
                                active_voice_channel = mpe_chan;
                                active_voice_key = k;
                            }
                        }
                    }
                },

                MpeEnableSwitch(false) => {

                    for k in cfg.keyrange.clone() {

                        if do_switch { break; }

                        if self.midi_unit.keystate(cfg.channel, k as u8) != 0
                        {
                            do_switch = true;
                            active_voice_key = k;
                            active_voice_channel = cfg.channel as usize;
                        }
                    }

                },
            }
            match do_switch {
                true => {
                    // confirm that no notes are active
                    voice.borrow_mut().uber_release();

                    if scene_non_ultra_release_voices == 0 {
                        play_configs.push(
                            PlayConfig {
                                active_voice_channel: active_voice_channel as i32,
                                active_voice_key:     active_voice_key as i32,
                                velocity:             cfg.velocity as i32,
                                lastdetune:           self.midi_unit.lastdetune(active_voice_channel as u8,lowkey) as u8,
                            }
                        );
                    }
                },
                false => {

                    if polymode != PolyMode::LatchMonophonic {
                        voice.borrow_mut().release(); 
                    }
                },
            }
        }

        for item in play_configs.iter() {
            self.play_voice(
                item.active_voice_channel, 
                item.active_voice_key as u8, 
                item.velocity as u8, 
                item.lastdetune,
            )?;
        }

        Ok(())
    }
}
