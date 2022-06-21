crate::ix!();

impl Phaser {

    pub fn do_phaser_block<const N: usize>(&mut self, 
        k: usize, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        self.feedback.process();

        self.d_l = data_l[k] + self.d_l * self.feedback.v;
        self.d_r = data_r[k] + self.d_r * self.feedback.v;

        self.d_l = limit_range(self.d_l, -32.0, 32.0);
        self.d_r = limit_range(self.d_r, -32.0, 32.0);

        self.d_l = self.biquad[0].process_sample(self.d_l);
        self.d_l = self.biquad[1].process_sample(self.d_l);
        self.d_l = self.biquad[2].process_sample(self.d_l);
        self.d_l = self.biquad[3].process_sample(self.d_l);
        self.d_r = self.biquad[4].process_sample(self.d_r);
        self.d_r = self.biquad[5].process_sample(self.d_r);
        self.d_r = self.biquad[6].process_sample(self.d_r);
        self.d_r = self.biquad[7].process_sample(self.d_r);

        self.l[k] = self.d_l;
        self.r[k] = self.d_r;
    }
}
