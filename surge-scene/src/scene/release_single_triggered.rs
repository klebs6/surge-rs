crate::ix!();

impl SurgeScene {

    pub fn release_single_triggered(&mut self, 
        voice_idx: usize,
        mut cfg: ReleaseCfg) 
    {
        let channel = self.voices[voice_idx].borrow().state.channel;
        let key     = self.voices[voice_idx].borrow().state.key;

        /* input these modes the note will collide on the main channel */
        let state_channel: u8 = 
            self.mpe_unit.get_mpe_main_channel( channel.try_into().unwrap(), key.try_into().unwrap());

        let note_channel:  u8 = self.mpe_unit.get_mpe_main_channel( cfg.channel, cfg.key );


        if (key, state_channel) == 
            (cfg.key as i32, note_channel ) 
        {
            cfg.do_release = true;

            // Again the note I need to legato to is on 
            // my channel in non MPE and is on another channel 
            // in MPE mode
            match self.mpe_unit.enabled() {
                MpeEnableSwitch(true) => {
                    self.release_single_triggered_mpe_enabled(voice_idx, &mut cfg);
                },
                MpeEnableSwitch(false) => {
                    self.release_single_triggered_mpe_disabled(voice_idx, &mut cfg);
                },
            }

            if cfg.do_release {
                self.voices[voice_idx].borrow_mut().release();
            }
        }
    }

    pub fn release_single_triggered_mpe_enabled(&mut self, 
        voice_idx: usize,
        cfg: &mut ReleaseCfg) 
    {
        let mut voice = self.voices[voice_idx].borrow_mut();

        // search downwards
        for k in cfg.keyrange.clone() { 

            if cfg.do_release { 
                break; 
            }

            for mpe_chan in 1_usize..16_usize {

                let nonzero_keystate: bool = 
                    self.midi_unit.keystate(mpe_chan as u8, k as u8) != 0;

                if mpe_chan != (cfg.channel as usize) && 
                    nonzero_keystate 
                {
                    let lastdetune = self.midi_unit.lastdetune(mpe_chan.try_into().unwrap(),k.try_into().unwrap());

                    voice.legato(k as i32, cfg.velocity as i32, lastdetune);

                    cfg.do_release = false;

                    // See the comment above at the other _st legato spot
                    voice.state.channel = mpe_chan as i32;

                    voice.state.voice_channel_state 
                        = self.midi_unit.channel_state_ptr(mpe_chan.try_into().unwrap());
                }
            }
        }
    }

    pub fn release_single_triggered_mpe_disabled(&mut self, 
        voice_idx: usize,
        cfg: &mut ReleaseCfg) 
    {
        let voice = &mut self.voices[voice_idx];

        let channel = cfg.channel as usize;

        for k in cfg.keyrange.clone() { // search downwards

            if !cfg.do_release { break; }

            if self.midi_unit.keystate(channel.try_into().unwrap(),k.try_into().unwrap()) != 0 {

                voice.borrow_mut().legato(k as i32, 
                    cfg.velocity as i32, 
                    self.midi_unit.lastdetune(channel.try_into().unwrap(),k.try_into().unwrap())
                );

                cfg.do_release = false;
                break;
            }
        }
    }
}

