crate::ix!();

//TODO: fix unknown coefficient names
coeffidx![ C; X0, X1, X2, X3, X4, X5, X6, X7 ];

#[derive(Derivative)] #[derivative(Debug)]
pub struct SvfFilter {
    #[derivative(Debug="ignore")]
    pub tuner:   TunerHandle,

    #[derivative(Debug="ignore")]
    pub srunit:  SampleRateHandle,

    pub pole_type:    PoleType,
    pub iirtype: FilterTypeIIR,
}

impl CoeffMake for SvfFilter {

    fn coeff_make(&self, freq: f32, mut reso: f32) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let f: f64 = CONCERT_A_HZ * self.tuner.n2p::<f64,true>(freq as f64);

        let f1: f64 = 2.0 * 
            ( PI * 
              std::cmp::min(
                  FloatOrd(0.11), 
                  FloatOrd(f * (0.25 * self.srunit.dsamplerate_inv()))
              ).0
            ).sin();// 4x oversampling

        reso = limit_range(reso, 0.0, 1.0).sqrt();

        let overshoot: f64 = match self.pole_type { 
            PoleType::FourPole => 0.1, 
            PoleType::TwoPole => 0.15 
        };

        let mut q1: f64 = { 
            2.0 - (reso as f64) * (2.0 + overshoot) + f1 * f1 * overshoot * 0.9 
        };

        q1 = {
            use std::cmp::min;
            let x = FloatOrd(2.00);
            let y = FloatOrd(2.00 - 1.52 * f1);
            let m: f64 = min(x, y).0;
            min(FloatOrd(q1), FloatOrd(m)).0
        };

        let clip_damp: f64 = 0.1 * (reso as f64) * f1;

        let a: f64 = 0.65;
        let gain: f64 = 1.0 - a * (reso as f64);

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];
        coeffs[0] = f1 as f32;
        coeffs[1] = q1 as f32;
        coeffs[2] = clip_damp as f32;
        coeffs[3] = gain as f32;
        coeffs
    }
}
