pub trait BiquadSetCoeffs {
    fn set_coef(&mut self, 
        a0: f64, 
        a1: f64, 
        a2: f64, 
        b0: f64, 
        b1: f64, 
        b2: f64);
}

pub trait BiquadCalcFreq {
    fn calc_omega(&self, scfreq: f64) -> f64;
    fn calc_omega_from_hertz(&self, hertz: f64) -> f64;
    fn calc_v1_q(&self, reso: f64) -> f64;
}

pub trait BiquadCoeffLP {
    fn coeff_lp(&mut self, omega: f64, quality_factor: f64);
    fn coeff_lp_with_bw(&mut self, omega: f64, bandwidth: f64);
}

pub trait BiquadCoeffLP2B {
    fn coeff_lp2b(&mut self, omega: f64, quality_factor: f64);
}

pub trait BiquadCoeffHP {
    fn coeff_hp(&mut self, omega: f64, quality_factor: f64);
    fn coeff_hp_with_bw(&mut self, omega: f64, bandwidth: f64);
}

pub trait BiquadCoeffBP {
    fn coeff_bp(&mut self, omega: f64, quality_factor: f64);
}

pub trait BiquadCoeffBP2A {
    fn coeff_bp2a(&mut self, omega: f64, quality_factor: f64);
}

pub trait BiquadCoeffPKA {
    fn coeff_pka(&mut self, omega: f64, quality_factor: f64);
}

pub trait BiquadCoeffNotch {
    fn coeff_notch(&mut self, omega: f64, quality_factor: f64);
}

pub trait BiquadCoeffEQ {
    fn coeff_peak_eq(&mut self, 
        omega: f64, 
        bandwidth: f64, 
        gain: f64);
}

pub trait BiquadCoeffAPF {
    fn coeff_apf(&mut self, omega: f64, quality_factor: f64);
}

pub trait BiquadCoeffLPHMorph {
    fn coeff_lph_morph(&mut self, omega: f64, quality_factor: f64, morph: f64);
}
