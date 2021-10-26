ix!();

use crate::{
    Distortion,
    DistortionParam,
};

impl Distortion {

    pub fn update_all_lp(&mut self) {
        let pre_hc:        f64 = self.pvalf(DistortionParam::PreHighCut).into();
        let post_hc:       f64 = self.pvalf(DistortionParam::PostHighCut).into();

        let pre_hc_omega:  f64 = self.lp1.calc_omega((pre_hc / 12.0) - 2.0);
        let post_hc_omega: f64 = self.lp2.calc_omega((post_hc / 12.0) - 2.0);

        self.lp1.coeff_lp2b(pre_hc_omega, 0.707);
        self.lp2.coeff_lp2b(post_hc_omega, 0.707);
    }

    pub fn update_all_bands(&mut self) {

        let prefreq:   f64 = self.pvalf(DistortionParam::PreFreq).into();
        let postfreq:  f64 = self.pvalf(DistortionParam::PostFreq).into();

        let omega1:    f64 = self.band1.calc_omega(prefreq / 12.0);
        let omega2:    f64 = self.band2.calc_omega(postfreq / 12.0);

        let pregain:   f64 = self.pvalf_extended(DistortionParam::PreGain).into();
        let postgain:  f64 = self.pvalf_extended(DistortionParam::PostGain).into();

        let prebw:     f64 = self.pvalf(DistortionParam::PreBandwidth).into();
        let postbw:    f64 = self.pvalf(DistortionParam::PostBandwidth).into();

        self.band1.coeff_peak_eq( omega1, prebw, pregain);
        self.band2.coeff_peak_eq( omega2, postbw,postgain);
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
