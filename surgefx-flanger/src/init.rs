crate::ix!();

impl Initialize for Flanger {

    fn init(&mut self) {

        for c in 0..2 {
            for i in 0..FLANGER_COMBS_PER_CHANNEL {
                self.lfophase[[c,i]] =  1.0 * 
                    ( (i as f32) + 0.5 * (c as f32) ) 
                    / ( FLANGER_COMBS_PER_CHANNEL as f32 );
            }

        }

        self.longphase = 0.0;

        for i in 0..FLANGER_LFO_TABLE_SIZE {
            self.sin_lfo_table[i] = 
                ( 2.0 * PI_32 * (i as f32) 
                  / (FLANGER_LFO_TABLE_SIZE as f32) 
                ).sin();
        }
    }
}

impl Flanger {
    pub fn new( 
        tuner:     & TunerHandle,
        tables:    & TablesHandle,
        srunit:    & SampleRateHandle,
        time_unit: & TimeUnitHandle) -> Self 
    {
        let mut x = Self {
            ringout:        Ringout::blocks(1024),
            params:         FlangerParam::new_runtime(), 
            idels:          [InterpDelay::new(), InterpDelay::new()],
            lfophase:       Self::new_lfo_phase(),
            longphase:      0.0,
            onepole_state:  OnePoleLPFilterState::default(),
            lfoval:         Self::new_comb_channels(),
            delaybase:      Self::new_comb_channels(),
            depth:          Default::default(),
            mix:            Default::default(),
            voices:         Default::default(),
            voice_detune:   Default::default(),
            voice_chord:    Default::default(),
            feedback:       Default::default(),
            fb_lf_damping:  Default::default(),
            stereo_width:   Default::default(),
            gain:           Default::default(),
            sin_lfo_table:  Self::new_lfo_table(),
            saw_lfo_table:  Self::new_lfo_table(),
            time_unit:      time_unit.clone(),
            tables:         tables.clone(),
            tuner:          tuner.clone(),
            srunit:         srunit.clone(),
        };
        x.init();
        x
    }

    #[inline] fn new_comb_channels() -> A2d::<LiPol::<f32>> {
        A2d::<LiPol::<f32>>::from_elem((2,FLANGER_COMBS_PER_CHANNEL), LiPol::<f32>::default())
    }

    #[inline] fn new_lfo_phase() -> A2d::<f32> {
        A2d::<f32>::zeros((2,FLANGER_COMBS_PER_CHANNEL))
    }

    #[inline] fn new_lfo_table() -> A1d::<f32> {
        A1d::<f32>::zeros(FLANGER_LFO_TABLE_SIZE)
    }
}
