crate::ix!();

impl<'plugin_layer> SurgeSynthesizer<'plugin_layer> {

    pub fn store_learn_custom(&mut self, cc_encoded: usize) {

        let learn_custom = self.midi_unit.learn_custom();

        if learn_custom >= 0 && 
            learn_custom < (N_CUSTOMCONTROLLERS as i32) 
        {
            self.controllers[learn_custom as usize] = cc_encoded as i32;
            self.midi_unit.save_midi_controllers();
            self.midi_unit.set_learn_custom(-1);
        }
    }

    pub fn maybe_switch_toggled(&mut self) {
        for scene in self.active_patch.scene.iter_mut() {
            scene.maybe_switch_toggled();
        }
    }

    pub fn channel_controller(&mut self, channel: u8, cc: u8, value: i32) {

        let mut fval: f32 = (value as f32) * (1.0 / 127.0);

        // store all possible NRPN & RPNs in a short array .. 
        // just amounts for 128kb or thereabouts anyway
        let return_now = match cc {
            0   => self.channel_controller_cc0(channel,cc,value),
            1   => self.channel_controller_cc1(channel,cc,fval),
            6   => self.channel_controller_cc6(channel,cc,value),
            10  => self.channel_controller_cc10(channel,cc,value),
            38  => self.channel_controller_cc38(channel,cc,value),
            64  => self.channel_controller_cc64(channel,cc,value),
            74  => self.channel_controller_cc74(channel,cc,value),
            98  => self.channel_controller_cc98(channel,cc,value),
            99  => self.channel_controller_cc99(channel,cc,value),
            100 => self.channel_controller_cc100(channel,cc,value),
            101 => self.channel_controller_cc101(channel,cc,value),

            /* all sound off */ 
            120 => { true },

            /* all notes off */ 
            123 => { true },

            /* no-op */ 
            _ => { false },
        };

        if return_now {
            return;
        }

        let cc_encoded = self.channel_controller_handle_rpn_nrpn(channel, cc, &mut fval);

        //self.maybe_send_cc_parameter_automation(cc_encoded, fval);
        //self.store_learn_param(cc_encoded);
        self.store_learn_custom(cc_encoded);
        //self.maybe_set_global_params_smoothed(cc_encoded,fval);
        //self.maybe_set_scene_params_smoothed(cc_encoded,fval);
    }
}
