crate::ix!();

pub trait BiquadCoeffEQ {

    /// takes in a radian frequency `omega`, bandwidth
    /// `bandwidth`, and gain `gain` as `f64` and sets
    /// the coefficients of an equalizer filter in the
    /// implementing struct.
    ///
    fn coeff_peak_eq(&mut self, 
        omega: f64, 
        bandwidth: f64, 
        gain: f64);
}

impl BiquadCoeffEQ for BiquadFilter {

    /// This is the implementation of the
    /// `coeff_peak_eq` method, which computes the
    /// coefficients for a peak equalization
    /// filter. 
    ///
    /// It takes in three parameters: `omega`,
    /// `bandwidth`, and `gain`. `omega` is the
    /// center frequency of the filter in radians
    /// per sample, `bandwidth` is the width of
    /// the filter in octaves, and `gain` is the
    /// boost or cut in decibels at the center
    /// frequency.
    ///
    fn coeff_peak_eq(&mut self, 
        omega: f64, mut bandwidth: f64, gain: f64)
    {
        // These lines define variables `g`, `gb`, and `g0`. `g` is the linear gain corresponding
        // to the input `gain`, `gb` is half of `g`, and `g0` is the unity gain.
        //
        let mut g: f64 = self.tables.db_to_linear(gain as f32).into();
        let gb:    f64 = self.tables.db_to_linear(gain as f32 * 0.5).into();
        let g0:    f64 = 1.0;

        // min(PI-0.000001,omega);
        let mut w0: f64 = omega; 

        // This line clamps the `bandwidth` parameter to a minimum value defined as `VLAG_MIN_BW`
        // and reassigns the result back to `bandwidth`.
        //
        bandwidth = std::cmp::max(FloatOrd(VLAG_MIN_BW), FloatOrd(bandwidth)).0;

        // This line computes `dww`, which is a intermediate value used later in the calculation of
        // the coefficients. 
        //
        // It is computed using the `sinh()` function, which calculates the hyperbolic sine.
        //
        // sinh = (e^x - e^-x)/2
        //
        let dww: f64 = 2.0 * w0 * ((2.0_f64.log10() / 2.0) * bandwidth).sinh(); 

        // This line checks if the difference between `g` and `g0` is greater than `0.00001`. 
        //
        // If it is, then the filter coefficients are computed. 
        //
        // Otherwise, the coefficients are set to unity gain.
        //
        if (g - g0).abs() > 0.00001
        {
            // These lines define variables used in the coefficient calculation. `gg_minus_gbgb` is
            // the absolute difference between `g^2` and `gb^2`, `g00` is the absolute difference
            // between `g^2` and `g0^2`, and `f00` is the absolute difference between `gb^2` and `g0
            //
            let gg_minus_gbgb: f64 = (g * g - gb * gb).abs();
            let mut g00:       f64 = (g * g - g0 * g0).abs();
            let mut f00:       f64 = (gb * gb - g0 * g0).abs();

            // In these lines of code, the numerator and denominator are calculated to obtain the
            // value of `g1`, which is used to compute the filter coefficients. 
            //
            // The numerator `num` is computed by multiplying `g0 * g0` with the difference between
            // `w0 * w0` and `PI * PI`, and then adding to it 
            //
            // `g * g` multiplied with `f00 * (PI * PI) * dww * dww` divided by `gg_minus_gbgb`. 
            //
            // The denominator `den` is calculated similarly, but instead of `g * g` it uses `f00`
            // and divides by `gg_minus_gbgb`. 
            //
            // Finally, `g1` is computed as the square root of the ratio of `num` and `den`.
            //
            let num:        f64 = 
                g0 * g0 * square(w0 * w0 - (PI * PI)) 
                + g * g * f00 * (PI * PI) * dww * dww / gg_minus_gbgb;

            let den:        f64 = 
                square(w0 * w0 - PI * PI) 
                + f00 * PI * PI * dww * dww / gg_minus_gbgb;

            let g1:         f64 = (num / den).sqrt();

            // These lines of code check if `omega` is greater than `PI`. 
            //
            // If it is, then 
            //
            // `g` is set to `g1 * 0.9999`, 
            //
            // `w0` is set to `PI - 0.00001`, 
            //
            // and `g00` and `f00` are recalculated. 
            //
            // After that, several variables are calculated based on the values of `g`, `gb`, `g0`,
            // and `g1`. 
            //
            // These variables are used in the following calculations to determine the filter
            // coefficients.
            //
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
