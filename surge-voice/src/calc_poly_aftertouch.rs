crate::ix!();

impl SurgeVoice {

    pub fn maybe_calc_poly_aftertouch(&mut self, 
        cfg: VoiceRuntimeHandle) 
    {
        let cfg = cfg.borrow();

        if cfg.do_poly_aftertouch 
        {
            let key      = self.state.key;

            if let Some(ModulationSource::ControllerModulationSource(ref mut x)) = 
                &mut self.modsources[ModSource::PolyphonicAfterTouch].as_deref_mut()
            {
                let target = 
                    self.mpe_unit.get_poly_aftertouch( key );

                x.set_target(target as f64);
                x.process_block();
            }
        }
    }
}
