crate::ix!();

impl ProcessBlockStereo for BiquadFilter {

    /**
      |# Safety
      |
      |data_l and data_r must each point to
      |BLOCK_SIZE valid contiguous data elements
      */
    unsafe fn process_block_stereo(&mut self, 
        data_l: *mut f32, 
        data_r: *mut f32, 
        out: Option<(*mut f32, *mut f32)>
    )
    {
        for k in 0..BLOCK_SIZE {

            self.a1.process();
            self.a2.process();
            self.b0.process();
            self.b1.process();
            self.b2.process();

            let mut input: f64 = *data_l.add(k) as f64;

            let mut op = input * self.b0.v[0] + self.reg0[0];

            self.reg0[0] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[0];
            self.reg1[0] = input * self.b2.v[0] - self.a2.v[0] * op;

            match out {
                Some(out) => *out.0.add(k)  = op as f32,
                None      => *data_l.add(k) = op as f32
            };

            input = *data_r.add(k) as f64;

            op = input * self.b0.v[0] + self.reg0[1];

            self.reg0[1] = input * self.b1.v[0] - self.a1.v[0] * op + self.reg1[1];
            self.reg1[1] = input * self.b2.v[0] - self.a2.v[0] * op;

            match out {
                Some(out) => *out.1.add(k)  = op as f32,
                None      => *data_r.add(k) = op as f32
            };
        }

        flush_denormal(&mut self.reg0[0]);
        flush_denormal(&mut self.reg1[0]);
        flush_denormal(&mut self.reg0[1]);
        flush_denormal(&mut self.reg1[1]);
    }
}
