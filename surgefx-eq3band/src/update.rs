crate::ix!();

const SCALE: f64 = 1.0 / 12.0;

impl Update for Eq3Band {

    fn update(&mut self) {
        self.update_all_bands(None);
    }
}

impl Eq3Band {

    #[inline] pub fn maybe_update(&mut self) {

        if self.block_increment == 0 {
            self.update();
        }

        self.block_increment = 
            (self.block_increment + 1) & SLOWRATE_M1 as i32;
    }

    #[inline] pub fn update_all_bands(&mut self, 
        gain_override: Option<f64>) 
    {
        self.update_band1(gain_override);
        self.update_band2(gain_override);
        self.update_band3(gain_override);
    }

    #[inline] pub fn coeff_instantize_all_bands(&mut self) {
        self.band1.coeff_instantize();
        self.band2.coeff_instantize();
        self.band3.coeff_instantize();
    }

    #[inline] pub fn instantize_unity_gain(&mut self) -> Option<f64> {
        let gain = 1.0_f64; // db_to_linear(fxdata->p[9].val.f));
        self.gain.set_target(gain as f32); 
        self.gain.instantize();
        Some(gain)
    }

    #[inline] pub fn update_gain(&mut self) {

        let gain_db = self.pvalf(Eq3BandParam::Gain);

        self.gain.set_target_smoothed(
            self.tables.db_to_linear(gain_db)
        );
    }

    #[inline] pub fn band_gain(&mut self, 
        gain_param:    Eq3BandParam,
        gain_override: Option<f64>) -> f64
    {
        match gain_override {

            // Set the bands to 0dB so the eq fades in initiallt
            Some(gain) => gain,

            None => self.pvalf(gain_param) as f64,
        }
    }

    #[inline] pub fn update_band1(&mut self, 
        gain_override: Option<f64>) 
    {
        let band1_freq  = self.pvalf(Eq3BandParam::LFreq) as f64;
        let band1_omega = self.band1.calc_omega(band1_freq * SCALE);
        let band1_width = self.pvalf(Eq3BandParam::LBandwidth) as f64;

        let gain = self.band_gain(Eq3BandParam::LGain, gain_override);

        self.band1.coeff_peak_eq( band1_omega, band1_width, gain); 
    }

    #[inline] pub fn update_band2(&mut self, 
        gain_override: Option<f64>) 
    {
        let band2_freq  = self.pvalf(Eq3BandParam::MFreq) as f64;
        let band2_omega = self.band2.calc_omega(band2_freq * SCALE);
        let band2_width = self.pvalf(Eq3BandParam::MBandwidth) as f64;

        let gain = self.band_gain(Eq3BandParam::MGain, gain_override);

        self.band2.coeff_peak_eq( band2_omega, band2_width, gain);
    }

    #[inline] pub fn update_band3(&mut self, 
        gain_override: Option<f64>) 
    {
        let band3_freq  = self.pvalf(Eq3BandParam::HFreq) as f64;
        let band3_omega = self.band3.calc_omega(band3_freq * SCALE);
        let band3_width = self.pvalf(Eq3BandParam::HBandwidth) as f64;

        let gain = self.band_gain(Eq3BandParam::HGain, gain_override);

        self.band3.coeff_peak_eq( band3_omega, band3_width, gain);
    }
}
