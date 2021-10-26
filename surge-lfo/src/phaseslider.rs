ix!();

use crate::{
    Lfo,
};

impl Lfo {

    pub fn get_phaseslider(
        &self, 
        lfo_shape: LfoShape, 
        start_phase: f32) -> f32 
    {
        let mut phaseslider: f32 = match lfo_shape {
            // Use Phase as shuffle-parameter instead
            LfoShape::StepSequencer => 0.0, 
            _                       => start_phase,
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
