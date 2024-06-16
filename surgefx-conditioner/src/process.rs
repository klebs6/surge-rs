crate::ix!();

macro_rules! write_vu_from_falloff {
    ($self:ident,$cfg:ident) => ({
        $self.vu[0] = minf(8.0, $cfg.a * $self.vu[0]);
        $self.vu[1] = minf(8.0, $cfg.a * $self.vu[1]);
        $self.vu[4] = minf(8.0, $cfg.a * $self.vu[4]);
        $self.vu[5] = minf(8.0, $cfg.a * $self.vu[5]);
    });
}

impl StereoProcess for Conditioner {

    fn stereo_process<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        let cfg = ConditionerProcessCfg::new(self);

        write_vu_from_falloff![self,cfg];

        self.update();

        self.process_bands(data_l,data_r);

        self.update_amp(&cfg);

        self.width.set_target_smoothed(clamp1_bipolar(cfg.width));

        self.postamp.set_target_smoothed(self.tables.db_to_linear(cfg.gain));

        self.process_mid_side(data_l,data_r);

        self.set_vu01(data_l, data_r);

        for k in 0..N {
            self.do_conditioner_block(
                k, 
                &cfg,
                data_l,
                data_r);
        }

        self.process_postamp(data_l, data_r);

        self.vu[2] = self.gain;

        self.set_vu45(data_l, data_r);
    }
}

impl Conditioner {

    #[inline] fn process_bands<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        unsafe {
            self.band1.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);

            self.band2.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);
        }
    }

    #[inline] fn update_amp(&mut self, cfg: &ConditionerProcessCfg) {

        self.amp_l.set_target_smoothed(
            cfg.pregain * 0.5 * clamp1_bipolar(1.0 - cfg.balance));

        self.amp_r.set_target_smoothed(
            cfg.pregain * 0.5 * clamp1_bipolar(1.0 + cfg.balance));
    }

    #[inline] fn process_postamp<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) {

        unsafe {
            self.postamp.multiply_2_blocks(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                block_size_quad![N]);
        }
    }

    #[inline] fn set_vu01<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) {

        self.vu[0] = maxf(
            self.vu[0], 
            unsafe {
                get_absmax(
                    data_l.as_mut_ptr(), 
                    block_size_quad![N]
                )
            }
        );

        self.vu[1] = maxf(
            self.vu[1], 
            unsafe {
                get_absmax(
                    data_r.as_mut_ptr(), 
                    block_size_quad![N]
                )
            }
        );
    }

    #[inline] fn set_vu45<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) {

        self.vu[4] = maxf(
            self.vu[4], 
            unsafe {
                get_absmax(
                    data_l.as_mut_ptr(), 
                    block_size_quad![N]
                )
            }
        );

        self.vu[5] = maxf(
            self.vu[5], 
            unsafe {
                get_absmax(
                    data_r.as_mut_ptr(), 
                    block_size_quad![N]
                )
            }
        );
    }

    #[inline] fn process_mid_side<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) {

        let mut mid_side = MSBlock::new(N);

        unsafe {

            encode_mid_side(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                mid_side.m(), 
                mid_side.s(), 
                block_size_quad![N]);

            self.width.multiply_block(mid_side.s(), block_size_quad![N]);

            decode_mid_side(
                mid_side.m(), 
                mid_side.s(), 
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                block_size_quad![N]);

            self.amp_l.multiply_block(
                data_l.as_mut_ptr(), 
                block_size_quad![N]);

            self.amp_r.multiply_block(
                data_r.as_mut_ptr(), 
                block_size_quad![N]);
        }
    }
}

impl ProcessOnlyControl for Conditioner {

    fn process_only_control<const N: usize>(&mut self) { 

        let cfg = ConditionerProcessCfg::new(self);

        write_vu_from_falloff![self,cfg];

        for _k in 0..N 
        {
            self.filtered_lamax  = (1.0 - cfg.attack)  * self.filtered_lamax  + cfg.attack;
            self.filtered_lamax2 = (1.0 - cfg.release) * self.filtered_lamax2 + ( cfg.release * self.filtered_lamax );

            if self.filtered_lamax > self.filtered_lamax2 {
                self.filtered_lamax2 = self.filtered_lamax;
            }

            self.gain = 1.0 / self.filtered_lamax2;
        }

        self.vu[2] = self.gain;
    }
}
