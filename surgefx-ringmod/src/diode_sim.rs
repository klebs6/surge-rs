ix!();

use crate::{RingModulator,RingModulatorParam};

impl RingModulator<'sr> {

    pub fn diode_sim(&self, v: f32) -> f32 {

        let forward_bias  = pvalf![self.params[RingModulatorParam::DiodeForwardBias]];
        let mut linear_region = pvalf![self.params[RingModulatorParam::DiodeLinearRegion]];

        let h = 1.0;

        linear_region = std::cmp::max( FloatOrd(linear_region), FloatOrd(forward_bias + 0.02) ).0;

        if v < forward_bias {
            return 0.0;
        }

        if v < linear_region {
            let vvb = v - forward_bias;
            return h * vvb * vvb 
                / ( 2.0 * linear_region - 2.0 * forward_bias );
        }

        let vlvb = linear_region - forward_bias;

        h * v - h * linear_region + h * vlvb * vlvb 
            / ( 2.0 * linear_region - 2.0 * forward_bias )
    }
}
