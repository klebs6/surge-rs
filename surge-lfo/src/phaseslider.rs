crate::ix!();

impl Lfo {

    /// This method takes in an `LfoShape` and a starting phase value and returns a phase slider
    /// value between 0 and 1. For all shapes except `LfoShape::StepSequencer`, the phase slider is
    /// equal to the starting phase value. For `LfoShape::StepSequencer`, the phase slider is
    /// always set to 0.
    /// 
    /// If the resulting phase slider value is less than 0, it is incremented by 1 until it is in
    /// the range [0, 1]. If it is greater than 1, it is decremented by 1 until it is in the range
    /// [0, 1].
    ///
    pub fn get_phaseslider(
        &self, 
        lfo_shape: LfoShape, 
        start_phase: f32) -> f32 
    {
        let mut phaseslider: f32 = match lfo_shape {
            // Use Phase as shuffle-parameter instead
            LfoShape::StepSequencer => 0.0, 
            _                       => start_phase
        };

        // With modulation the phaseslider 
        // can be outside [0,1], as in #1524
        while phaseslider < 0.0 {
            phaseslider += 1.0;
        }

        while phaseslider > 1.0 {
            phaseslider -= 1.0;
        }

        phaseslider
    }
}
