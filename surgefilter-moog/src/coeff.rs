ix!();

impl CoeffMake for crate::LpMoogFilter<'sr> {

    fn coeff_make(&self, freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS]
    {
        let note_pitch_ignoring_tuning = 
            self.tuner.n2p::<f64,true>(freq as f64);

        let gg: f64 = limit_range(
            (
                CONCERT_A_HZ as f64 * 
                note_pitch_ignoring_tuning * 
                self.srunit.dsamplerate_os_inv()
            ) as f32, 
            0.0, 0.187).into(); // gg

        let t_b1: f32 = (1.0 - (-2.0 * PI * gg).exp()) as f32;

        let q: f32 = std::cmp::min(
            FloatOrd(2.15 * limit_range(reso, 0.0, 1.0)), 
            FloatOrd(0.5 / (t_b1 * t_b1 * t_b1 * t_b1))
        ).0;

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];
        coeffs[0] = 3.0 / (3.0 - q);
        coeffs[1] = t_b1;
        coeffs[2] = q;
        coeffs
    }
}
