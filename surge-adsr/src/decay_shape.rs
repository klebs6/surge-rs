crate::ix!();

pub enum AdsrEnvelopeDecayShape {
    Linear,
    Quadratic,
    Cubic,
}

pub trait GetDecayShape {

    type DecayShape;

    fn decay_shape(&self) -> Self::DecayShape;
}

impl GetDecayShape for AdsrEnvelope {

    type DecayShape = AdsrEnvelopeDecayShape;

    fn decay_shape(&self) -> Self::DecayShape {

        match pvali![self.params[AdsrParam::DecayShape]] {
            1 => AdsrEnvelopeDecayShape::Quadratic,
            2 => AdsrEnvelopeDecayShape::Cubic,
            _ => AdsrEnvelopeDecayShape::Linear,
        }
    }
}

pub trait GetDecayShapeBounds {

    fn get_decay_shape_bounds(&self, rate: f32) -> (f32,f32);
}

pub trait GetLinearDecayShapeBounds {

    /// This function computes the lower and upper bounds for the decay
    /// stage of the ADSR envelope when using a linear decay shape. The
    /// function takes the decay rate `rate` as an argument, and returns
    /// a tuple of two values, which are the lower and upper bounds of
    /// the decay stage.
    /// 
    fn get_linear_decay_shape_bounds(&self, rate: f32) -> (f32,f32);
}

pub trait GetQuadraticDecayShapeBounds {

    /// This function computes the lower and upper bounds for the decay
    /// stage of the ADSR envelope when using a quadratic decay
    /// shape. The function takes the decay rate `rate` as an argument,
    /// and returns a tuple of two values, which are the lower and upper
    /// bounds of the decay stage.
    ///     
    fn get_quadratic_decay_shape_bounds(&self, rate: f32) -> (f32,f32);
}

pub trait GetCubicDecayShapeBounds {

    /// This function computes the lower and upper bounds for the decay stage of the ADSR envelope
    /// when using a cubic decay shape. 
    ///
    /// The function takes the decay rate `rate` as an argument, and returns a tuple of two values,
    /// which are the lower and upper bounds of the decay stage.
    /// 
    fn get_cubic_decay_shape_bounds(&self, rate: f32) -> (f32,f32);
}
