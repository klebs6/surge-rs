ix!();

use crate::{
    ProcessBlockMono,
    BiquadFilter,
};

impl ProcessBlockMono for BiquadFilter<'sr> {

    unsafe fn process_block_mono(
        &mut self, 
        data: *mut f32, 
        out: Option<*mut f32>)
    {
        for k in 0..BLOCK_SIZE {

            self.a1.process();
            self.a2.process();
            self.b0.process();
            self.b1.process();
            self.b2.process();

            let input: f64 = *data.add(k) as f64;

            let op = input * self.b0.v[0] + self.reg0[0];

            self.reg0[0] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];
            self.reg1[0] = input * self.b2.v[0] - self.a2.v[0] * op;

            match out {
                Some(out) => {
                    *out.add(k) = op as f32;
                },
                None => {
                    *data.add(k) = op as f32;
                },
            }
        }

        flush_denormal(&mut self.reg0[0]);
        flush_denormal(&mut self.reg1[0]);
    }
}

