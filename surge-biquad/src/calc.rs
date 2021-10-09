ix!();

use crate::{
    BiquadFilter,
    BiquadCalcFreq,
};

impl BiquadCalcFreq for BiquadFilter<'sr> {

    fn calc_omega(&self, scfreq: f64) -> f64 {
        (2.0 * PI) 
            * CONCERT_A_HZ
            * self.tuner.n2p::<true,f64>((12.0 * scfreq) as f64) 
            * self.srunit.dsamplerate_inv() 
    }

    fn calc_omega_from_hertz(&self, hertz: f64) -> f64 {
        (2.0 * PI) 
            * hertz * self.srunit.dsamplerate_inv() 
    }

    fn calc_v1_q(&self, reso: f64) -> f64 {
        1.0 / (1.02 - limit_range(reso as f32, 0.0_f32, 1.0_f32) as f64)
    }
}

impl PlotMagnitude for BiquadFilter<'sr> {

    fn plot_magnitude(&self, freq: f32) -> f32 {
        let ca0 = Complex64::new(1.0, 0.0);
        let ca1 = Complex64::new(self.a1.v[0], 0.0);
        let ca2 = Complex64::new(self.a2.v[0], 0.0);
        let cb0 = Complex64::new(self.b0.v[0], 0.0);
        let cb1 = Complex64::new(self.b1.v[0], 0.0);
        let cb2 = Complex64::new(self.b2.v[0], 0.0);
        let i   = Complex64::new(0.0, 1.0);

        let z: Complex64 = (-2.0 * PI * (freq as f64) * i).exp();

        let h: Complex64 = (cb0 + cb1 * z + cb2 * z * z) / (ca0 + ca1 * z + ca2 * z * z);

        let r: f64 = h.to_polar().0;
        r as f32
    }
}
