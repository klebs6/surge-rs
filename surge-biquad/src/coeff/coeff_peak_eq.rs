ix!();

use crate::{
    BiquadSetCoeffs,
    BiquadCoeffEQ,
    BiquadFilter,
};

impl BiquadCoeffEQ for BiquadFilter {

    fn coeff_peak_eq(&mut self, 
        omega: f64, mut bandwidth: f64, gain: f64)
    {
        let mut g: f64 = self.tables.db_to_linear(gain as f32).into();
        let gb:    f64 = self.tables.db_to_linear(gain as f32 * 0.5).into();
        let g0:    f64 = 1.0;

        // min(PI-0.000001,omega);
        let mut w0: f64 = omega; 

        bandwidth = std::cmp::max(FloatOrd(VLAG_MIN_BW), FloatOrd(bandwidth)).0;

        // sinh = (e^x - e^-x)/2
        let dww: f64 = 2.0 * w0 * ((2.0_f64.log10() / 2.0) * bandwidth).sinh(); 

        if (g - g0).abs() > 0.00001
        {
            let gg_minus_gbgb: f64 = (g * g - gb * gb).abs();
            let mut g00:       f64 = (g * g - g0 * g0).abs();
            let mut f00:       f64 = (gb * gb - g0 * g0).abs();

            let num:        f64 = 
                g0 * g0 * square(w0 * w0 - (PI * PI)) 
                + g * g * f00 * (PI * PI) * dww * dww / gg_minus_gbgb;

            let den:        f64 = 
                square(w0 * w0 - PI * PI) 
                + f00 * PI * PI * dww * dww / gg_minus_gbgb;

            let g1:         f64 = (num / den).sqrt();

            if omega > PI
            {
                g = g1 * 0.9999;
                w0 = PI - 0.00001;
                g00 = (g * g - g0 * g0).abs();
                f00 = (gb * gb - g0 * g0).abs();
            }

            let gg:  f64 = g * g;
            let gbgb: f64 = gb * gb;
            let g01:       f64 = (gg  - g0 * g1).abs();
            let g11:       f64 = (gg  - g1 * g1).abs();
            let f01:       f64 = (gbgb - g0 * g1).abs();

            // blir wacko ?  goes crazy (?)
            let f11: f64 = (gbgb - g1 * g1).abs(); 

            let w2: f64 = 
                (g11 / g00).sqrt() * square((w0 / 2.0).tan());

            let w_lower: f64 = 
                w0 * 2.0_f64 .powf( -0.5 * bandwidth);

            let w_upper: f64 = 
                2.0 * (
                    (f00 / f11).sqrt() * (g11 / g00).sqrt() * square((w0 / 2.0).tan()) 
                    / (w_lower / 2.0).tan()
                ).atan();


            let dw: f64 = {
                let diff: f64 = (w_upper - w_lower).abs();
                (1.0 + (f00 / f11).sqrt() * w2) * (diff / 2.0).tan()
            };

            let c: f64 = f11 * dw * dw - 2.0 * w2 * (f01 - (f00 * f11).sqrt());

            let d: f64 = 2.0 * w2 * (g01 - (g00 * g11).sqrt());

            let scaled_root_c_plus_d: f64 = ((c + d) / gg_minus_gbgb).sqrt();

            let b: f64 = ((gg * c + gbgb * d) / gg_minus_gbgb).sqrt();

            let a0: f64 = 1.0 + w2 + scaled_root_c_plus_d;
            let a1: f64 = -2.0 * (1.0 - w2);
            let a2: f64 = 1.0 + w2 - scaled_root_c_plus_d;
            let b0: f64 = g1 + g0 * w2 + b;
            let b1: f64 = -2.0 * (g1 - g0 * w2);
            let b2: f64 = g1 - b + g0 * w2;

            self.set_coef(a0, a1, a2, b0, b1, b2);

        } else {

            self.set_coef(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        }
    }
}
