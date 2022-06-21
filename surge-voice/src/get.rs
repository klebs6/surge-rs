crate::ix!();

impl SurgeVoice {

    #[inline(always)] pub fn get_table_pitch(&mut self, 
        idx: i32) -> f32 
    {
        self.tuner.get_tablepitch(idx) as f32
    }

    #[inline] pub fn get_cutoff(&self, idx: usize, cfg: &VoiceUpdateQFCSCfg ) -> f32 {

        let keytrack: f32 = (self.state.pitch as f32) - (cfg.keytrack_root as f32);

        let fenv: f32 = self.modsources[ModSource::FilterEg].as_ref().unwrap().get_output() as f32;

        let f_cutoff: f32 = cfg.filterunit_cutoff[idx];

        let f_keytrack: f32 = cfg.filterunit_keytrack[idx];

        let f_envmod: f32 = cfg.filterunit_envelopemode[idx];

        f_cutoff + (f_keytrack * keytrack) + (f_envmod * fenv)
    }

    ///TODO!
    pub fn get_generator_pair(&self) -> (Box<dyn CoeffMake>, Box<dyn CoeffMake>) 
    {
        todo!("need to write this function for set_qfb");
    }

    #[inline] pub fn get_cutoff_pair(&self, cfg: &VoiceUpdateQFCSCfg) -> (f32, f32) 
    {
        let a = self.get_cutoff(0, cfg);
        let mut b = self.get_cutoff(1, cfg);

        if cfg.f2_cutoff_is_offset {
            b += a;
        }
        (a, b)
    }

    #[inline] pub fn get_reso_pair(&self, cfg: &VoiceUpdateQFCSCfg) -> (f32, f32) 
    {
        let a = cfg.filterunit_resonance[0];
        let b = cfg.filterunit_resonance[1];
        (a, b)
    }

    #[inline] pub fn get_ft_pair(&self, cfg: &VoiceUpdateQFCSCfg) -> (FilterType, FilterType) 
    {
        let a = cfg.filterunit_type[0];
        let b = cfg.filterunit_type[1];
        (a, b)
    }

    #[inline] pub fn get_ft_subtype_pair(&self, cfg: &VoiceUpdateQFCSCfg) -> (FilterSubType, FilterSubType) 
    {
        let a = cfg.filterunit_subtype[0];
        let b = cfg.filterunit_subtype[1];
        (a, b)
    }
}
