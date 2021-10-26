ix!();

use crate::{
    ProcessSampleStereo,
    BiquadFilter,
};

impl ProcessSampleStereo for BiquadFilter {

    #[inline] fn process_sample_stereo(&mut self, 
        l: f32, r: f32, l_out: &mut f32, r_out: &mut f32)
    {
        self.coeff_process();

        let mut input: f64 = l as f64;

        let mut op = input * self.b0.v[0] + self.reg0[0];

        self.reg0[0] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];
        self.reg1[0] = input * self.b2.v[0] - self.a2.v[0] * op;

        *l_out = op as f32;

        input = r as f64;

        op = input * self.b0.v[0] + self.reg0[1];

        self.reg0[1] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];
        self.reg1[1] = input * self.b2.v[0] - self.a2.v[0] * op;

        *r_out = op as f32;
    }
}
