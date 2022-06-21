crate::ix!();

impl SurgeScene {

    pub fn runtime_create_octave(&self) -> [f32; 3] {
        [
            self.oscillator_octave(0),
            self.oscillator_octave(1),
            self.oscillator_octave(2),
        ]
    }

    pub fn runtime_create_pitch(&self) -> [f32; 3] {
        [
            self.oscillator_pitch(0) as f32,
            self.oscillator_pitch(1) as f32,
            self.oscillator_pitch(2) as f32,
        ]
    }

    pub fn runtime_create_absolute(&self) -> [bool; 3] {
        [
            self.oscillator_pitch_absolute(0),
            self.oscillator_pitch_absolute(1),
            self.oscillator_pitch_absolute(2),
        ]
    }

    pub fn runtime_extend_range(&self) -> [bool; 3] {
        [
            self.oscillator_pitch_extend_range(0),
            self.oscillator_pitch_extend_range(1),
            self.oscillator_pitch_extend_range(2),
        ]
    }

    pub fn runtime_voice_update_qfcs_cfg(&self) -> VoiceUpdateQFCSCfg {
        VoiceUpdateQFCSCfg {
            f2_cutoff_is_offset:      self.f2_cutoff_is_offset(),
            feedback:                 self.feedback(),
            filterbalance:            self.filter_balance(),
            filterblock_cfg:          self.filterblock_cfg(),
            filterunit_cutoff:        vec![
                self.filterunit_cutoff(0),
                self.filterunit_cutoff(1),
            ],
            filterunit_envelopemode:  vec![
                self.filterunit_envelopemode(0),
                self.filterunit_envelopemode(1),
            ],
            filterunit_keytrack:      vec![
                self.filterunit_keytrack(0),
                self.filterunit_keytrack(1),
            ],
            filterunit_resonance:     vec![
                self.filterunit_resonance(0),
                self.filterunit_resonance(1),
            ],
            filterunit_subtype:       vec![
                self.filterunit_filtersubtype(0),
                self.filterunit_filtersubtype(1),
            ],
            filterunit_type:          vec![
                self.filterunit_filtertype(0),
                self.filterunit_filtertype(1),
            ],
            keytrack_root:            self.keytrack_root() as f32,
            vca_level:                self.vca_level(),
            vca_velsense:             self.vca_velsense(),
            waveshaper0_drive_db:     self.waveshaper_drive(0),
        }
    }

    pub fn runtime_pitchbend_cfg(&self) -> PitchBendCfg {
        PitchBendCfg{
            range_up:   self.pitchbend_range_up() as f32,
            range_down: self.pitchbend_range_down() as f32,
        }
    }

    pub fn runtime_voice_toggle_solo_cfg(&self) -> VoiceToggleSoloCfg {
        VoiceToggleSoloCfg {
            oscillator0_solo: self.oscillator_solo(0),
            oscillator1_solo: self.oscillator_solo(1),
            oscillator2_solo: self.oscillator_solo(2),
            noise0_solo:      self.noise_solo(0),
            ring0_solo:       self.ring_solo(0),
            ring1_solo:       self.ring_solo(1),
            oscillator0_mute: self.oscillator_mute(0),
            oscillator1_mute: self.oscillator_mute(1),
            oscillator2_mute: self.oscillator_mute(2),
            noise_mute:       self.noise_mute(0),
            ring0_mute:       self.ring_mute(0),
            ring1_mute:       self.ring_mute(1),
        }
    }

    pub fn runtime_filterunit_filtertype(&self) -> Vec<FilterType> {
        vec![
            self.filterunit_filtertype(0),
            self.filterunit_filtertype(1),
        ]
    }

    pub fn runtime_filterunit_filtersubtype(&self) -> Vec<FilterSubType> {
        vec![
            self.filterunit_filtersubtype(0),
            self.filterunit_filtersubtype(1),
        ]
    }

    pub fn runtime_oscillator_types(&self) -> [OscillatorType; N_OSCS] {
        [
            self.oscillator_type(0),
            self.oscillator_type(1),
            self.oscillator_type(2),
        ]
    }

    pub fn create_voice_runtime(&mut self) -> VoiceRuntimeHandle {
        Rc::new(RefCell::new(VoiceRuntime {
            filterblock_cfg:          self.filterblock_cfg(),
            drift:                    self.drift(),
            fm_depth:                 self.fm_depth(),
            noise_colour:             self.noise_colour(),
            oscillator_octave:        self.runtime_create_octave(),
            oscillator_pitch:         self.runtime_create_pitch(),
            oscillator_absolute:      self.runtime_create_absolute(),
            oscillator_extend_range:  self.runtime_extend_range(),
            oscillator_keytrack2:     self.oscillator_keytrack(2),
            voice_update_qfcs_cfg:    self.runtime_voice_update_qfcs_cfg(),
            update_lfo:               [true; 6],
            modulation_voice:         self.modulation_voice(),
            modulation_scene:         self.modulation_scene(),
            portamento:               self.portamento(),
            portamento_min:           self.portamento_min(),
            portamento_temposync:     self.portamento_temposync(),
            pitch:                    self.pitch(),
            octave:                   self.octave(),
            pitch_extend_range:       self.pitch_extend_range(),
            pitchbend_cfg:            self.runtime_pitchbend_cfg(),
            oscillator_level0:        self.oscillator_level(0),
            oscillator_level1:        self.oscillator_level(1),
            oscillator_level2:        self.oscillator_level(2),
            noise_level0:             self.noise_level(0),
            ring_level0:              self.ring_level(0),
            ring_level1:              self.ring_level(1),
            oscillator_route0:        self.oscillator_route(0),
            oscillator_route1:        self.oscillator_route(1),
            oscillator_route2:        self.oscillator_route(2),
            ring_route0:              self.ring_route(0),
            ring_route1:              self.ring_route(1),
            noise_route0:             self.noise_route(0),
            keytrack_root:            self.keytrack_root(),
            do_poly_aftertouch:       self.do_poly_aftertouch(),
            pan:                      self.pan(),
            width:                    self.width(),
            volume:                   self.volume(),
            filterunit_filtertype:    self.runtime_filterunit_filtertype(),
            filterunit_filtersubtype: self.runtime_filterunit_filtersubtype(),
            voice_toggle_solo_cfg:    self.runtime_voice_toggle_solo_cfg(),
            oscillator_type:          self.runtime_oscillator_types(),
            fm_cfg:                   self.fm_cfg(),
        }))
    }

    pub fn switch_toggled(&mut self) {

        let voice_runtime = self.create_voice_runtime();

        for voice in self.voices.iter_mut() {
            voice.borrow_mut().switch_toggled(voice_runtime.clone());
        }
    }

    pub fn all_notes_off(&mut self) {

        let mut to_free = vec![];

        for (idx, _v) in self.voices.iter_mut().enumerate() {
            to_free.push(idx);
        }

        for idx in to_free.iter() {
            self.free_voice(*idx);
        }

        self.voices.clear();
    }

    #[inline] pub fn playscene(&self) -> bool {
        self.voices.is_empty()
    }

    pub fn do_routing_critical_secion(
        &mut self, 
        mut fb_entry: i32, 
        mut vcount:   i32) -> (i32, i32) {

        let mut to_free = vec![];

        let voice_runtime = self.create_voice_runtime();

        for (idx, voice) in self.voices.iter_mut().enumerate() {

            let resume: ShouldKeepPlaying = voice.borrow_mut().process_block(
                voice_runtime.clone(), 
                &mut self.fbq.state[(fb_entry >> 2) as usize], 
                fb_entry & 3
            );

            fb_entry += 1;
            vcount += 1;

            if !resume {
                to_free.push(idx);
            } 
        }

        for idx in to_free.iter() {
            self.free_voice(*idx);
        }

        (fb_entry, vcount)
    }

    pub fn post_process(&mut self) {

        unsafe {
            hardclip_block8(
                self.out.buf[0].as_mut_ptr(), 
                BLOCK_SIZE_OS_QUAD);

            hardclip_block8(
                self.out.buf[1].as_mut_ptr(), 
                BLOCK_SIZE_OS_QUAD);

            self.halfband.process_block_downsample_by_two(
                self.out.buf[0].as_mut_ptr(), 
                self.out.buf[1].as_mut_ptr(), None, None, None);
        }

    }

    pub fn apply_highpass(&mut self) {

        let lowcut = pvalf![self.params[SceneParam::LowCut]];

        let omega  = self.highpass.calc_omega( lowcut as f64 / 12.0);

        // var 0.707
        self.highpass.coeff_hp( omega, 0.4); 

        let l: *mut f32 = self.out.buf[0].as_mut_ptr();
        let r: *mut f32 = self.out.buf[1].as_mut_ptr();

        unsafe {
            // TODO: quadify
            self.highpass.process_block_stereo(l,r, None); 
        }
    }

    pub fn process(&mut self, fb_entry: i32) {

        let fts1 = self.get_filtertype(0);
        let fts2 = self.get_filtertype(1);
        let wst  = self.get_waveshape_type(0);

        let mut g = FbqGlobal {
            fu1ptr: Some(get_quad_filter_ptr(fts1,Some(self.get_filtersubtype(0)))),
            fu2ptr: Some(get_quad_filter_ptr(fts2,Some(self.get_filtersubtype(1)))),
            wsptr:  Some(get_quad_filter_waveshaper_ptr(wst)),
        };

        let fbconfig           = self.get_fbconfig();
        let fn_process_quad_fb = get_fn_process_quad(fbconfig, &g);

        let mut waveshaper_state = WaveshaperState {
            tables: self.tables.clone(),
        };

        for e in (0..fb_entry).step_by(4) {

            let units: i32 = fb_entry - e;

            let voice_idx = (e >> 2) as usize;

            for idx in units..4 {
                let idx = idx as usize;
                self.fbq.state[voice_idx].unit_state[0].active[idx] = 0;
                self.fbq.state[voice_idx].unit_state[1].active[idx] = 0;
                self.fbq.state[voice_idx].unit_state[2].active[idx] = 0;
                self.fbq.state[voice_idx].unit_state[3].active[idx] = 0;
            }

            fn_process_quad_fb(
                &mut waveshaper_state,
                &mut self.fbq.state[voice_idx], 
                &mut g, 
                self.out.buf[0].as_mut_ptr(), 
                self.out.buf[1].as_mut_ptr()
            );
        }

        let sync_registers_from_qfb_cfg = self.voice_sync_registers_from_qfb_cfg();

        for voice in self.voices.iter_mut() {
            // save filter state in voices after quad processing is done
            voice.borrow_mut().sync_registers_from_qfb(&sync_registers_from_qfb_cfg); 
        }
    }

    pub fn voice_sync_registers_from_qfb_cfg(&self) -> SyncQFBRegistersCfg {
        todo!();
    }
}
