crate::ix!();

impl SurgeVoice {

    #[inline] pub fn init_modsources(&mut self) {
        self.init_lfos();
        self.init_aftertouch();
        self.init_velocitysource();
        self.init_releasevelocitysource();
        self.init_keytracksource();
        self.init_amp_eg();
        self.init_filter_eg();
        self.init_channel_aftertouch();
        self.init_modsource_timbre();
    }

    #[inline] pub fn init_aftertouch(&mut self) {

        if let Some(ModulationSource::ControllerModulationSource(ref mut aftertouch)) = 
            &mut self.modsources[ModSource::AmpEg].as_deref_mut()
        { 
            let key      = self.state.key;

            let value = self.mpe_unit.get_poly_aftertouch( key ) as f64;
            aftertouch.init(value);
        }
    }

    #[inline] pub fn init_velocitysource(&mut self) {

        if let Some(ModulationSource::ControllerModulationSource(ref mut velocity_source)) = 
            &mut self.modsources[ModSource::Velocity].as_deref_mut()
        { 
            velocity_source.output = self.state.fvel;
        }
    }

    #[inline] pub fn init_releasevelocitysource(&mut self) {

        if let Some(ModulationSource::ControllerModulationSource(ref mut release_velocity_source)) = 
            &mut self.modsources[ModSource::ReleaseVelocity].as_deref_mut()
        { 
            release_velocity_source.output = self.state.freleasevel;
        }
    }

    #[inline] pub fn init_keytracksource(&mut self) {

        if let Some(ModulationSource::ControllerModulationSource(ref mut keytrack_source)) = 
            &mut self.modsources[ModSource::KeyTrack].as_deref_mut()
        { 
            keytrack_source.output = 0.0;
        }
    }

    #[inline] pub fn init_amp_eg(&mut self) {

        if let Some(ModulationSource::AdsrEnvelope(ref mut amp_eg)) = 
            &mut self.modsources[ModSource::AmpEg].as_deref_mut()
        { 
            amp_eg.init();
        }
    }

    #[inline] pub fn init_filter_eg(&mut self) {

        if let Some(ModulationSource::AdsrEnvelope(ref mut filter_eg)) = 
            &mut self.modsources[ModSource::FilterEg].as_deref_mut()
        { 
            filter_eg.init();
        }
    }

    #[inline] pub fn init_channel_aftertouch(&mut self) {

        if let Some(ModulationSource::ControllerModulationSource(ref mut mono_aftertouch)) = 
            &mut self.modsources[ModSource::ChannelAfterTouch].as_deref_mut()
        { 
            unsafe {
                mono_aftertouch.output = (*self.state.voice_channel_state).pressure as f64;
            }
        }
    }

    #[inline] pub fn init_modsource_timbre(&mut self) {

        if let Some(ModulationSource::ControllerModulationSource(ref mut timbre_source)) = 
            &mut self.modsources[ModSource::Timbre].as_deref_mut()
        { 
            unsafe {
                timbre_source.output = (*self.state.voice_channel_state).timbre as f64;
            }
        }
    }

    #[inline] pub fn init_lfos(&mut self) {

        for idx in 0..6 {

            //maybe also need to configure stepsequences from patch and scene
            //maybe also need to configure with voice state
            let lfo = Lfo::new(self.time_unit.clone(), self.tables.clone());

            let ms = ModSource::voice_lfo(idx);

            self.modsources[ms] = Some(Box::new(ModulationSource::Lfo(lfo)));
        }
    }
}
