crate::ix!();

pub struct VoiceConstructor {
    pub timeunit:                 TimeUnitHandle,
    pub tables:                   TablesHandle,
    pub tuner:                    TunerHandle,
    pub srunit:                   SampleRateHandle,
    pub synth_in:                 SynthInputHandle,
    pub mpe_unit:                 MPEUnitHandle,
    pub key:                      i32,
    pub velocity:                 i32,
    pub channel:                  i32,
    pub detune:                   f64,
    pub keystate:                 *mut MidiKeyState,
    pub main_channel_state:       *mut MidiChannelState,
    pub voice_channel_state:      *mut MidiChannelState,
    pub mpe_enabled:              MpeEnableSwitch,
    pub fm_cfg:                   FmConfiguration,
    pub polymode:                 PolyMode,
    pub filterunit_filtertype:    Vec<FilterType>,
    pub filterunit_filtersubtype: Vec<FilterSubType>,
    pub voice_toggle_solo_cfg:    VoiceToggleSoloCfg,
    pub oscillator_type:          [OscillatorType; N_OSCS],
    pub voice_runtime:            VoiceRuntimeHandle,
}

impl SurgeVoice {

    pub fn new_voice_state(cfg: &VoiceConstructor) -> SurgeVoiceState {
        SurgeVoiceState {
            pitch:                SurgeVoiceState::default_pitch(),
            pkey:                 SurgeVoiceState::default_pkey(),
            key:                  cfg.key,
            channel:              cfg.channel,
            velocity:             cfg.velocity,
            fvel:                 (cfg.velocity as f64) / 127.0,
            releasevelocity:      0,
            freleasevel:          0.0,
            detune:               cfg.detune,
            uberrelease:          false,
            keystate:             cfg.keystate,
            main_channel_state:   cfg.main_channel_state,
            voice_channel_state:  cfg.voice_channel_state,
            portaphase:           0.0,
            gate:                 true,
            keep_playing:         true,
            portasrc_key:         Default::default(),
        }
    }

    pub fn new_coeffmakers(cfg: &VoiceConstructor) -> [FilterCoefficientMaker; 2] {
        [
            FilterCoefficientMaker::new(
                cfg.tuner.clone(),
                cfg.tables.clone(),
                cfg.srunit.clone()
            ),
            FilterCoefficientMaker::new(
                cfg.tuner.clone(),
                cfg.tables.clone(),
                cfg.srunit.clone()
            ),
        ]
    }

    pub fn new(cfg: VoiceConstructor) -> Result<Self,SurgeError>
    {
        let mut voice = Self {
            output:              Align16([[0.0; BLOCK_SIZE_OS]; 2]),
            osclevels:           Align16(create_voice_osclevels()),
            fmbuffer:            Align16([0.0; BLOCK_SIZE_OS]),
            osctype:             [OscillatorType::Off; N_OSCS],
            state:               Self::new_voice_state(&cfg),
            age:                 0,
            age_release:         0,
            filterblock_state:   FilterBlockState::default(),
            fbp:                 FBP::default(),
            coeffmaker:          Self::new_coeffmakers(&cfg),
            route:               [0; 6],
            octave_size:         12.0, // 12.0
            osc_enable:          [false; 3],
            ring_enable:         [false; 2],
            noise_enable:        false,
            fm_mode:             FmConfiguration::Off,
            noisegen_l:          [0.0; 2],
            noisegen_r:          [0.0; 2],
            osc:                 create_voice_oscillators(cfg.tuner.clone()),
            filterblock_data:    FilterBlockData::default(),
            mpe_enabled:         cfg.mpe_enabled,
            uuid:                Uuid::new_v4(),

            modsources:         create_voice_modsources(
                cfg.timeunit.clone(),
                cfg.tables.clone(),
                cfg.srunit.clone()
            )?,

            tables:             cfg.tables.clone(),
            time_unit:          cfg.timeunit.clone(),
            tuner:              cfg.tuner.clone(),
            srunit:             cfg.srunit.clone(),
            synth_in:           cfg.synth_in.clone(),
            mpe_unit:           cfg.mpe_unit.clone(),
        };

        voice.init_portasrc( &cfg );

        voice.mpe_unit.set_lastkey(cfg.key);

        voice.init_modsources()?;

        //AmpEg, FilterEg, Voice_LFO[1..6]
        voice.modsource_attack();

        // init interpolators
        //voice.calc_ctrldata::<true>(&cfg.calc_ctrldata_cfg, None, 0); 
        //todo

        {
            let voice_runtime = cfg.voice_runtime.borrow();
            // init Quad-Filterblock parameter interpolators
            voice.set_quad_filterblock(&voice_runtime.voice_update_qfcs_cfg, None, 0); 
        }

        // It is imposrtant that switch_toggled comes last since it 
        // creates and activates the oscillators which need the 
        // modulation state set in calc_ctrldata to get sample 0 right.
        voice.switch_toggled(cfg.voice_runtime.clone())?;

        Ok(voice)
    }
}
