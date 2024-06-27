crate::ix!();

impl<'plugin_layer> SurgeSynthesizer<'plugin_layer> {

    #[inline] pub fn channel_controller_cc0(&mut self, _channel: u8, _cc: u8, value: i32) -> bool {
        self.cc0 = value;
        true
    }

    #[inline] pub fn channel_controller_cc1(&mut self, _channel: u8, _cc: u8, fval: f32) -> bool {
        for scene in self.active_patch.scene.iter_mut() {

            match &mut scene.modsources[ModSource::ModWheel].as_deref_mut() {

                Some(ModulationSource::ControllerModulationSource(ref mut inner)) => inner.set_target(fval as f64),
                _ => unreachable!(),
            }
        }
        false
    }

    #[inline] pub fn channel_controller_cc6(&mut self, channel: u8, _cc: u8, value: i32) -> bool {

        let do_nrpn = self.midi_unit.nrpn_last(channel);

        match do_nrpn {
            true => {

                self.midi_unit.set_nrpn_v(channel,1,value);

                let nrpn0   = self.midi_unit.get_nrpn(channel,0);
                let nrpn1   = self.midi_unit.get_nrpn(channel,1);
                let nrpn_v0 = self.midi_unit.get_nrpn_v(channel,0);
                let nrpn_v1 = self.midi_unit.get_nrpn_v(channel,1);

                self.on_nrpn(channel,  nrpn0, nrpn1, nrpn_v0, nrpn_v1);
            },
            false => {

                self.midi_unit.set_rpn_v(channel,1,value);

                let rpn0   = self.midi_unit.get_rpn(channel,0);
                let rpn1   = self.midi_unit.get_rpn(channel,1);
                let rpn_v0 = self.midi_unit.get_rpn_v(channel,0);
                let rpn_v1 = self.midi_unit.get_rpn_v(channel,1);

                self.on_rpn(channel, rpn0, rpn1, rpn_v0, rpn_v1 );
            },
        }
        true
    }

    #[inline] pub fn channel_controller_cc10(&mut self, channel: u8, _cc: u8, value: i32) -> bool {
        match self.mpe_unit.enabled() {
            MpeEnableSwitch(true) => {
                self.midi_unit.set_pan(channel, int7_to_bipolar_float(value));
                true
            },
            MpeEnableSwitch(false) => false,
        }
    }

    #[inline] pub fn channel_controller_cc38(&mut self, channel: u8, _cc: u8, value: i32) -> bool {
        match self.midi_unit.nrpn_last(channel) {
            true  => self.midi_unit.set_nrpn_v(channel,0,value),
            false => self.midi_unit.set_rpn_v(channel,0,value),
        }
        false
    }

    #[inline] pub fn channel_controller_cc64(&mut self, channel: u8, _cc: u8, value: i32) -> Result<bool,SurgeError> {

        let scenemode = SceneMode::try_from(
            pvali![ self.active_patch.params[PatchParam::SceneMode] ] as usize).unwrap();

        self.midi_unit.set_hold(channel,value > 63); // check hold pedal

        // OK in single mode, only purge scene 0, 
        // but in split or dual purge both, 
        // and in chsplit pick based on channel
        match scenemode {
            SceneMode::Single => {

                let scene_active = 
                    pvali![self.active_patch.params[PatchParam::SceneActive]] as usize;

                self.active_patch.scene[scene_active].purge_holdbuffer()?;
            },
            SceneMode::KeySplit | SceneMode::Dual => {
                self.active_patch.scene[0].purge_holdbuffer()?;
                self.active_patch.scene[1].purge_holdbuffer()?;
            },
            SceneMode::ChannelSplit => {

                // a control channel message
                match (self.mpe_unit.enabled(), channel) {
                    (MpeEnableSwitch(true), 0) => {
                        self.active_patch.scene[0].purge_holdbuffer()?;
                        self.active_patch.scene[1].purge_holdbuffer()?;
                    },
                    _ => {

                        let splitkey = pvali![self.active_patch.params[PatchParam::SplitKey]];

                        if channel < (((( splitkey / 8 ) as i32) + 1 ) as u8) {
                            self.active_patch.scene[0].purge_holdbuffer()?;
                        } else {
                            self.active_patch.scene[1].purge_holdbuffer()?;
                        }
                    }
                }
            },
        }

        Ok(true)
    }

    #[inline] pub fn channel_controller_cc74(&mut self, channel: u8, _cc: u8, value: i32) -> bool {
        match self.mpe_unit.enabled() {
            MpeEnableSwitch(true) => {
                self.midi_unit.set_timbre(channel,int7_to_bipolar_float(value));
                true
            },
            _ => false,
        }
    }

    #[inline] pub fn channel_controller_cc98(&mut self, channel: u8, _cc: u8, value: i32) -> bool {
        // NRPN LSB
        self.midi_unit.set_nrpn(channel,0,value);
        self.midi_unit.set_nrpn_last(channel,true);
        true
    }

    #[inline] pub fn channel_controller_cc99(&mut self, channel: u8, _cc: u8, value: i32) -> bool {
        // NRPN MSB
        self.midi_unit.set_nrpn(channel,0,value);
        self.midi_unit.set_nrpn_last(channel,true);
        true
    }

    #[inline] pub fn channel_controller_cc100(&mut self, channel: u8, _cc: u8, value: i32) -> bool {
        // RPN LSB
        self.midi_unit.set_rpn(channel,0,value);
        self.midi_unit.set_nrpn_last(channel,false);
        true
    }

    #[inline] pub fn channel_controller_cc101(&mut self, channel: u8, _cc: u8, value: i32) -> bool {
        // RPN MSB
        self.midi_unit.set_rpn(channel,1,value);
        self.midi_unit.set_nrpn_last(channel,false);
        true
    }
}
