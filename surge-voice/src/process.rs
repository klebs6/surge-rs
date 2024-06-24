crate::ix!();

impl SurgeVoice {

    pub fn clear_output_stereo(&mut self) -> Result<(),AlignmentError> {

        unsafe {
            clear_block::<BLOCK_SIZE_OS_QUAD>(self.output[0].as_mut_ptr())?;
            clear_block::<BLOCK_SIZE_OS_QUAD>(self.output[1].as_mut_ptr())?;
        }

        Ok(())
    }

    pub fn process_block(
        &mut self, 
        cfg: VoiceRuntimeHandle,
        qfcs:   &mut QuadFilterChainState, 
        qfcs_idx:  i32

    ) -> Result<ShouldKeepPlaying,AlignmentError> {

        self.calc_ctrldata::<false>(cfg.clone(), Some(qfcs), qfcs_idx);

        self.clear_output_stereo();

        let osc2_or_ring1          = self.osc_enable[2] || self.ring_enable[1];
        let osc0or1                = self.osc_enable[0] || self.osc_enable[1];
        let fm2to1to0              = self.fm_mode == FmConfiguration::TwoToOneToZero;
        let fm1and2to0             = self.fm_mode == FmConfiguration::OneAndTwoToZero;
        let osc0or1_and_fm2to1to0  = osc0or1 && fm2to1to0;
        let osc1_and_fm1and2to0    = self.osc_enable[0]  && fm1and2to0;

        let gate2: bool =  {
            osc2_or_ring1 
                || osc0or1_and_fm2to1to0 
                || osc1_and_fm1and2to0 
        };

        let gate1: bool = {
            self.osc_enable[1] 
                || self.ring_enable[0] 
                || self.ring_enable[1] 
                || (self.fm_mode.on() && self.osc_enable[0])
        };

        let mut runtime = self.gen_oscillator_runtime(cfg.clone());

        if gate2 {
            self.process_osc(&mut runtime,2);
        }

        if gate1 {
            self.process_osc(&mut runtime,1);
        }


        if self.osc_enable[0] || self.ring_enable[0] {

            let osc_1l = match self.osc[1] { Some(ref mut x) => x.out_l(), _ => panic!(), };
            let osc_2l = match self.osc[2] { Some(ref mut x) => x.out_l(), _ => panic!(), };

            if fm1and2to0 {
                add_block(
                    osc_1l, 
                    osc_2l, 
                    self.fmbuffer.as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD
                )?;
            }

            self.process_osc(&mut runtime,0);
        }

        if self.ring_enable[0] {
            self.process_ring(&mut runtime,0);
        }

        if self.ring_enable[1] {
            self.process_ring(&mut runtime,1);
        }

        if self.noise_enable {
            self.process_noise(&mut runtime);
        }

        unsafe {
            // PFG
            self.osclevels[LagEntry::Pfg].multiply_2_blocks(
                self.output[0].as_mut_ptr(), 
                self.output[1].as_mut_ptr(), 
                BLOCK_SIZE_OS_QUAD);
        }

        for i in 0..BLOCK_SIZE_OS {
            unsafe {

                //TODO: question: is qfcs_idx a pointer offset??
                //this could crash if not.  we should figure out a safer way to do this
                _mm_store_ss(
                    qfcs.dl.as_mut_ptr().offset(i as isize + qfcs_idx as isize) as *mut f32,
                    _mm_load_ss(self.output[0].as_mut_ptr().add(i)));

                _mm_store_ss(
                    qfcs.dr.as_mut_ptr().offset(i as isize + qfcs_idx as isize) as *mut f32,
                    _mm_load_ss(self.output[1].as_mut_ptr().add(i)));
            }
        }

        let cfg = cfg.borrow();

        self.set_quad_filterblock(&cfg.voice_update_qfcs_cfg, Some(qfcs), qfcs_idx);

        self.age += 1;

        if !self.state.gate {
            self.age_release += 1;
        }

        Ok(ShouldKeepPlaying(self.state.keep_playing))
    }
}
