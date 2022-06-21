crate::ix!();

impl SurgeScene {

    pub fn reset_ctrl_modsources(&mut self) {
        for i in 0..N_CUSTOMCONTROLLERS {
            let ms = ModSource::ctrl(i as usize);
            if let Some(ref mut src) = self.modsources[ms] {
                src.reset();
            }
        }
    }

    pub fn process_scene_lfos(&mut self) {
        for i in 0..N_LFOS_PER_SCENE 
        {
            let ms = ModSource::scene_lfo(i);
            self.process_modsource(ms);
        }
    }

    pub fn process_ctrl_modsources(&mut self) {
        for i in 0..N_CUSTOMCONTROLLERS
        {
            let ms = ModSource::ctrl(i);
            self.process_modsource(ms);
        }
    }

    pub fn process_modsource(&mut self, ms: ModSource) {
        if let Some(ref mut src) = self.modsources[ms] {
            src.process_block();
        }
    }

    #[inline] pub fn toggle_scene_modsource(
        &mut self, ms: ModSource, val: bool) 
    {
        if let Some(ref mut src) = self.modsources[ms] {
            src.enable(val);
        }
    }

    #[inline] pub fn disable_all_scene_modsources(&mut self) {
        for modsource in 0..ModSource::count() 
        {
            let modsource = ModSource::try_from(modsource).unwrap();

            self.toggle_scene_modsource(modsource,false); 
        }
    }

    pub fn prepare_to_process_modsources(&mut self)
    {
        self.disable_all_scene_modsources();

        let mut doprocess_queue: Vec::<ModSource> = Vec::new();

        for routing in self.modulation_scene.iter_mut() {
            let src = routing.src;
            doprocess_queue.push(src);
        }

        for routing in self.modulation_voice.iter_mut() {
            let src = routing.src;
            doprocess_queue.push(src);
        }

        for modsrc in doprocess_queue.iter() {
            self.toggle_scene_modsource(*modsrc,true);
        }
    }

    pub fn process_modsources(&mut self) {
        self.prepare_to_process_modsources();
        self.process_modsource(ModSource::ModWheel);
        self.process_modsource(ModSource::ChannelAfterTouch);
        self.process_modsource(ModSource::PitchBend);
        self.process_ctrl_modsources();
        self.apply_scene_modulation();
        self.process_scene_lfos();
    }

    pub fn apply_scene_modulation(&mut self) {
        for i in 0_usize..self.modulation_scene.len() 
        {
            let src: ModSource = self.modulation_scene[i].src;

            if let Some(_modsrc) = &self.modsources[src] 
            {
                let modsource   = self.modsources[src].as_ref().unwrap();

                let src_output = 
                    modsource.get_output() as f32;

                let depth: f32 = 
                    self.modulation_scene[i].depth as f32;

                let dst = 
                    &mut self.modulation_scene[i].dst;

                let dst_f01 = dst.borrow().get_value_f01();

                let new_val = PData::Float(dst_f01 + depth * src_output);

                dst.borrow_mut().set_modulation_val(new_val);
            }
        }
    }
}

/**
  | want to conform to something like this
  | interface
  |
  */
#[test] fn modulation_if() {

   /* 
    let envelope = scene.modsources[ModSource::Amp_EG];

    let depth = 0.1;

    let routing = scene.fx_unit.fx[FXType::Distortion].params[DistortionParam::Gnar].add_modsource(envelope, depth);

    self.routings.push(routing);
   */

}
