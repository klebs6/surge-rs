ix!();

use crate::{
    ProcessSampleNolag,
    ProcessSampleStereoNolag,
    ProcessSampleNolagNoinput,
    BiquadFilter,
};

impl ProcessSampleNolag for BiquadFilter {

    #[inline] fn process_sample_nolag(&mut self, 
        l: &mut f32, r: &mut f32)
    {
        let mut op = *l as f64 * self.b0.v[0] + self.reg0[0];

        self.reg0[0] = *l as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];
        self.reg1[0] = *l as f64 * self.b2.v[0] - self.a2.v[0] * op;

        *l = op as f32;

        op = *r as f64 * self.b0.v[0] + self.reg0[1];

        self.reg0[1] = *r as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];
        self.reg1[1] = *r as f64 * self.b2.v[0] - self.a2.v[0] * op;

        *r = op as f32;
    }
}

impl ProcessSampleStereoNolag for BiquadFilter {

    #[inline] fn process_sample_stereo_nolag(&mut self, 
        l: &mut f32, r: &mut f32, l_out: &mut f32, r_out: &mut f32)
    {
        let mut op = *l as f64 * self.b0.v[0] + self.reg0[0];

        self.reg0[0] = *l as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];
        self.reg1[0] = *l as f64 * self.b2.v[0] - self.a2.v[0] * op;

        *l_out = op as f32;

        op = *r as f64 * self.b0.v[0] + self.reg0[1];

        self.reg0[1] = *r as f64 * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];
        self.reg1[1] = *r as f64 * self.b2.v[0] - self.a2.v[0] * op;

        *r_out = op as f32;
    }
}

impl ProcessSampleNolagNoinput for BiquadFilter {

    #[inline] fn process_sample_nolag_noinput(&mut self, 
        l_out: &mut f32, r_out: &mut f32)
    {
        let mut op = self.reg0[0];

        self.reg0[0] = -self.a1.v[0] * op + self.reg1[0];
        self.reg1[0] = -self.a2.v[0] * op;

        *l_out = op as f32;

        op = self.reg0[1];

        self.reg0[1] = -self.a1.v[0] * op + self.reg1[1];
        self.reg1[1] = -self.a2.v[0] * op;

        *r_out = op as f32;
    }
}
