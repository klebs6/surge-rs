crate::ix!();

impl Process for DualDelay {

    fn process<const N: usize>(&mut self, 
        data_l: &mut [f32; N], data_r: &mut [f32; N]) 
    {
        self.update();

        self.maybe_instantize_all();

        self.clear_scratch_buffers();

        for k in 0..BLOCK_SIZE {
            self.do_dualdelay_block( k );
        }

        unsafe {
            self.do_softclip();

            self.do_filters();

            self.do_pan(data_l, data_r);

            self.do_feedback();

            self.do_crossfeed();

            self.dualdelay_set_buffer();

            self.do_mid_side();

            self.do_mix(data_l,data_r);
        }

        self.update_wpos();
    }
}

impl DualDelay {

    #[inline] pub fn clear_scratch_buffers(&mut self) {
        self.scratch_left.fill(0.0);
        self.scratch_right.fill(0.0);
        self.wetblock.clear();
    }

    #[inline] pub unsafe fn do_softclip(&mut self ) {

        softclip_block(
            self.scratch_left.as_mut_ptr(),  
            BLOCK_SIZE_QUAD
        );

        softclip_block(
            self.scratch_right.as_mut_ptr(), 
            BLOCK_SIZE_QUAD
        );
    }

    #[inline] pub unsafe fn do_filters(&mut self) {

        self.lp.process_block_stereo(
            self.scratch_left.as_mut_ptr(), 
            self.scratch_right.as_mut_ptr(), 
            None
        );

        self.hp.process_block_stereo(
            self.scratch_left.as_mut_ptr(), 
            self.scratch_right.as_mut_ptr(), 
            None
        );
    }

    #[inline] pub unsafe fn do_pan<const N: usize>(&mut self, 
        data_l: &mut [f32; N],
        data_r: &mut [f32; N]
    ) {
        self.pan.trixpan_blocks(
            data_l.as_mut_ptr(), 
            data_r.as_mut_ptr(), 
            self.wetblock.l(), 
            self.wetblock.r(), 
            BLOCK_SIZE_QUAD);
    }

    #[inline] pub unsafe fn do_feedback(&mut self) {
        self.feedback.mac_2_blocks_to(
            self.scratch_left.as_mut_ptr(), 
            self.scratch_right.as_mut_ptr(), 
            self.wetblock.l(), 
            self.wetblock.r(), 
            BLOCK_SIZE_QUAD);
    }

    #[inline] pub unsafe fn do_crossfeed(&mut self) {
        self.feedback.mac_2_blocks_to(
            self.scratch_left.as_mut_ptr(), 
            self.scratch_right.as_mut_ptr(), 
            self.wetblock.r(), 
            self.wetblock.l(), 
            BLOCK_SIZE_QUAD);
    }

    #[inline] pub unsafe fn do_mid_side(&mut self) {

        // scale width
        let mut ms = MSBlock::new(BLOCK_SIZE);

        encode_mid_side(
            self.scratch_left.as_mut_ptr(), 
            self.scratch_right.as_mut_ptr(), 
            ms.m(), 
            ms.s(), 
            BLOCK_SIZE_QUAD);

        self.width.multiply_block(
            ms.s(), 
            BLOCK_SIZE_QUAD);

        decode_mid_side(
            ms.m(), 
            ms.s(), 
            self.scratch_left.as_mut_ptr(), 
            self.scratch_right.as_mut_ptr(), 
            BLOCK_SIZE_QUAD);
    }

    #[inline] pub unsafe fn do_mix<const N: usize>(&mut self, 
        data_l: &mut [f32; N],
        data_r: &mut [f32; N]
    ) {
        self.mix.fade_2_blocks_to(
            data_l.as_mut_ptr(), 
            self.scratch_left.as_mut_ptr(), 
            data_r.as_mut_ptr(), 
            self.scratch_right.as_mut_ptr(), 
            data_l.as_mut_ptr(), 
            data_r.as_mut_ptr(), 
            BLOCK_SIZE_QUAD);
    }

    #[inline] pub fn update_wpos(&mut self) {
        self.wpos += BLOCK_SIZE as i32;
        self.wpos &= DUAL_DELAY_MAX_DELAY_LENGTH as i32 - 1;
    }

    #[inline] pub fn maybe_instantize_all(&mut self) {
        if !self.inithadtempo 
            && self.time_unit.temposyncratio_inv() != 0.0 
        {
            self.instantize_all();
            self.inithadtempo = true;
        } 
    }
}

impl ProcessOnlyControl for DualDelay {
    fn process_only_control<const N: usize>(&mut self) 
    { 
        todo!();
    }
}
