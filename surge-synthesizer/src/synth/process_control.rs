crate::ix!();

impl<'plugin_layer> SurgeSynthesizer<'plugin_layer> {

    pub fn maybe_suspend_fx(&mut self) -> Result<(),SurgeError>
    {
        if self.fx_unit.fx_suspend_bitmask != 0
        {
            for i in 0..8 {

                let masked: bool = ((1 << i) & self.fx_unit.fx_suspend_bitmask) != 0;

                if masked 
                {
                    self.fx_unit.fx[i].suspend()?
                }

            }
            self.fx_unit.fx_suspend_bitmask = 0;
        }

        Ok(())
    }

    pub fn maybe_load_fx(&mut self) -> Result<(),SurgeError> {
        if self.fx_unit.load_fx_needed 
        {
            self.fx_unit.load_fx(false, false)?;
        }

        Ok(())
    }

    #[inline] pub fn get_scene_active_mask(&self) -> i32 {
        pvali![self.active_patch.params[PatchParam::SceneActive]]
    }

    #[inline] pub fn get_play_scenes(&self) -> (bool,bool) {

        let scene_mode   = self.get_scene_mode();
        let scene_active = self.get_scene_active_mask();

        let sm_gate = 
            (scene_mode == SceneMode::KeySplit)     || 
            (scene_mode == SceneMode::Dual)         || 
            (scene_mode == SceneMode::ChannelSplit);

        let play_a: bool = sm_gate || (scene_active == 0);
        let play_b: bool = sm_gate || (scene_active == 1);
        (play_a,play_b)
    }

    pub fn interpolate_midi_controllers(&mut self) { }

    pub fn process_control(&mut self) -> Result<(),SurgeError> {

        let (play_a, play_b) = self.get_play_scenes();

        self.timeunit.update();

        for scene in self.active_patch.scene.iter_mut() {
            scene.maybe_release(play_a)?;
            scene.maybe_release(play_b)?;
        }

        /*
          note: channelmask was causing problems
          so i took it out we can figure out
          a different way to do this later
          */
        if !self.controller.halt_engine {

            self.active_patch.scene[0].maybe_play_note(play_a, 1)?;

            self.active_patch.scene[1].maybe_play_note(play_b, 2)?;

        }

        self.interpolate_midi_controllers();

        if play_a {
            self.active_patch.scene[0].process_modsources();
        }

        if play_b {
            self.active_patch.scene[1].process_modsources();
        }

        self.maybe_switch_toggled()?;
        self.maybe_load_fx()?;
        self.maybe_suspend_fx()?;

        Ok(())
    }
}
