crate::ix!();

impl GetDecayShapeBounds for AdsrEnvelope {

    fn get_decay_shape_bounds(&self, rate: f32) -> (f32,f32) {

        match self.decay_shape() {
            AdsrEnvelopeDecayShape::Linear    => self.get_linear_decay_shape_bounds(rate),
            AdsrEnvelopeDecayShape::Quadratic => self.get_quadratic_decay_shape_bounds(rate),
            AdsrEnvelopeDecayShape::Cubic     => self.get_cubic_decay_shape_bounds(rate),
        }
    }
}

impl GetLinearDecayShapeBounds for AdsrEnvelope {

    /// The function computes the lower bound as `phase - rate`, and
    /// the upper bound as `phase + rate`. The phase value
    /// represents the current level of the envelope, and the rate
    /// represents the rate at which the envelope decays during the decay
    /// stage.
    ///
    fn get_linear_decay_shape_bounds(&self, rate: f32) -> (f32,f32) {

        let phase   = self.phase();

        let lo = phase - rate;
        let hi = phase + rate;

        (lo,hi)
    }
}

impl GetQuadraticDecayShapeBounds for AdsrEnvelope {

    // The function computes the lower bound as `phase - 2.0 * sx
    // * rate + rate * rate`,
    //
    // and the upper bound as `phase + 2.0 * sx * rate + rate
    // * rate`, 
    //
    // where `sx` is the square root of the phase value. 
    //
    // These formulas represent a quadratic curve that starts at the
    // current phase value and reaches zero at the sustain level.
    // 
    // There is also a special case handled in this function, where if
    // the sustain level is very low (less than `1e-3`) and the phase is
    // close to zero (less than `1e-4`), the lower bound is set to zero,
    // to avoid the envelope going above the sustain level during the
    // decay stage.
    //
    fn get_quadratic_decay_shape_bounds(&self, rate: f32) -> (f32,f32) {

        let phase   = self.phase();
        let sustain = self.get_sustain_parameter();

        let sx: f32 = phase.sqrt();

        let mut l_lo = phase - 2.0 * sx * rate + rate * rate;
        let     l_hi = phase + 2.0 * sx * rate + rate * rate;

        // That + rate * rate in both means at low
        // sustain ( < 1e-3 or so) you end up with
        // lo and hi both pushing us up off
        // sustain. 
        //
        // Unfortunatley we ned to handle that
        // case specially by pushing lo down
        if sustain < 1e-3 && phase < 1e-4 {
            l_lo = 0.0;
        } 

        (l_lo,l_hi)
    }
}

impl GetCubicDecayShapeBounds for AdsrEnvelope {

    /// The function computes the lower bound as
    ///
    /// `phase - 3.0 * sx_sx_rate + 3.0 * sx_rate_rate - rate_cubed`,
    ///
    /// and the upper bound as 
    ///
    /// `phase + 3.0 * sx_sx_rate + 3.0 * sx_rate_rate + rate_cubed`, 
    ///
    /// where `sx` is the cube root of the phase value. 
    ///
    /// These formulas represent a cubic curve that starts at the current phase value and reaches
    /// zero at the sustain level.
    /// 
    fn get_cubic_decay_shape_bounds(&self, rate: f32) -> (f32,f32) {

        let phase = self.phase();

        let sx: f32 = phase.powf(0.3333333);

        let three_sx_sx_rate   = 3.0 * sx * sx * rate;
        let three_sx_rate_rate = 3.0 * sx * rate * rate;
        let rate_cubed         = rate * rate * rate;

        let l_lo = phase - three_sx_sx_rate + three_sx_rate_rate - rate_cubed;
        let l_hi = phase + three_sx_sx_rate + three_sx_rate_rate + rate_cubed;

        (l_lo,l_hi)
    }
}
