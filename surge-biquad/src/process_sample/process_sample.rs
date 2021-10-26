ix!();

use crate::{
    ProcessSample,
    BiquadFilter,
};

impl ProcessSample for BiquadFilter {

    #[inline] fn process_sample(&mut self, input: f32) -> f32 
    {
        self.coeff_process();

        let op = (input as f64) * self.b0.v[0] + self.reg0[0] ;

        self.reg0[0] = input as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];
        self.reg1[0] = input as f64 * self.b2.v[0] - self.a2.v[0] * op;

        op as f32
    }
}
