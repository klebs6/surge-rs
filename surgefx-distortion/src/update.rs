crate::ix!();

impl Distortion {

    #[inline] pub fn update_drive(&mut self, drive: f32) {
        self.drive.set_target_smoothed(self.tables.db_to_linear(drive));
    }

    #[inline] pub fn update_outgain(&mut self, outgain: f32) {
        self.outgain.set_target_smoothed(self.tables.db_to_linear(outgain));
    }

    #[inline] pub fn maybe_update(&mut self) {

        // TODO fix denormals!
        if self.block_increment == 0 {
            self.update();
        }

        self.block_increment = (self.block_increment + 1) & SLOWRATE_M1 as i32;
    }

    #[inline] pub fn update_pre_hc(&mut self) {
        let pre_hc:        f64 = self.pvalf(DistortionParam::PreHighCut).into();
        let pre_hc_omega:  f64 = self.lp1.calc_omega((pre_hc / 12.0) - 2.0);

        self.lp1.coeff_lp2b(pre_hc_omega, 0.707);
    }

    #[inline] pub fn update_post_hc(&mut self) {
        let post_hc:       f64 = self.pvalf(DistortionParam::PostHighCut).into();
        let post_hc_omega: f64 = self.lp2.calc_omega((post_hc / 12.0) - 2.0);

        self.lp2.coeff_lp2b(post_hc_omega, 0.707);
    }

    #[inline] pub fn update_all_lp(&mut self) {
        self.update_pre_hc();
        self.update_post_hc();
    }

    #[inline] pub fn update_band1(&mut self) {
        let prefreq: f64 = self.pvalf(DistortionParam::PreFreq).into();
        let omega1:  f64 = self.band1.calc_omega(prefreq / 12.0);
        let pregain: f64 = self.pvalf_extended(DistortionParam::PreGain).into();
        let prebw:   f64 = self.pvalf(DistortionParam::PreBandwidth).into();

        self.band1.coeff_peak_eq( omega1, prebw, pregain);
    }

    #[inline] pub fn update_band2(&mut self) {
        let postfreq:  f64 = self.pvalf(DistortionParam::PostFreq).into();
        let omega2:    f64 = self.band2.calc_omega(postfreq / 12.0);
        let postgain:  f64 = self.pvalf_extended(DistortionParam::PostGain).into();
        let postbw:    f64 = self.pvalf(DistortionParam::PostBandwidth).into();

        self.band2.coeff_peak_eq( omega2, postbw,postgain);
    }

    #[inline] pub fn update_all_bands(&mut self) {
        self.update_band1();
        self.update_band2();
    }

    pub fn zero_drives(&mut self) {
        self.drive.set_target(0.0);
        self.outgain.set_target(0.0);
    }

    pub fn coeff_instantize_all_lp(&mut self) {
        self.lp1.coeff_instantize();
        self.lp2.coeff_instantize();
    }
}

impl Update for Distortion {

    fn update(&mut self) {
        self.update_all_bands();
        self.update_all_lp();
        self.coeff_instantize_all_lp();
    }
}
