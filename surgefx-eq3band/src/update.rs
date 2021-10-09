ix!();

use crate::{
    Eq3BandParam,
    Eq3Band,
};

impl Update for Eq3Band<'sr> {

    fn update(&mut self) {
        self.update_all_bands(None);
    }
}

impl Eq3Band<'sr> {

    pub fn coeff_instantize_all_bands(&mut self) {
        self.band1.coeff_instantize();
        self.band2.coeff_instantize();
        self.band3.coeff_instantize();
    }

    pub fn instantize_unity_gain(&mut self) -> Option<f64> {
        let gain = 1.0_f64; // db_to_linear(fxdata->p[9].val.f));
        self.gain.set_target(gain as f32); 
        self.gain.instantize();
        Some(gain)
    }

    pub fn update_all_bands(&mut self, 
        gain_override: Option<f64>) 
    {
        let scale = 1.0 / 12.0;

        let band1_freq = self.pvalf(Eq3BandParam::LFreq) as f64;
        let band2_freq = self.pvalf(Eq3BandParam::MFreq) as f64;
        let band3_freq = self.pvalf(Eq3BandParam::HFreq) as f64;

        let band1_omega = self.band1.calc_omega(band1_freq * scale);
        let band2_omega = self.band2.calc_omega(band2_freq * scale);
        let band3_omega = self.band3.calc_omega(band3_freq * scale);

        let band1_width = self.pvalf(Eq3BandParam::LBandwidth) as f64;
        let band2_width = self.pvalf(Eq3BandParam::MBandwidth) as f64;
        let band3_width = self.pvalf(Eq3BandParam::HBandwidth) as f64;

        match gain_override {
            None => {

                let band1_gain = self.pvalf(Eq3BandParam::LGain) as f64;
                let band2_gain = self.pvalf(Eq3BandParam::MGain) as f64;
                let band3_gain = self.pvalf(Eq3BandParam::HGain) as f64;

                self.band1.coeff_peak_eq( band1_omega, band1_width, band1_gain );
                self.band2.coeff_peak_eq( band2_omega, band2_width, band2_gain );
                self.band3.coeff_peak_eq( band3_omega, band3_width, band3_gain );
            },
            Some(gain) => {
                // Set the bands to 0dB so the eq fades in initiallt
                self.band1.coeff_peak_eq( band1_omega, band1_width, gain); 
                self.band2.coeff_peak_eq( band2_omega, band2_width, gain);
                self.band3.coeff_peak_eq( band3_omega, band3_width, gain);
            }
        }
    }
}
