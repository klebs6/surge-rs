ix!();

impl crate::AllpassVerb<'sr> {

    pub fn calc_size(&mut self, scale: f32) {

        let m: f32 = scale;

        assert!(m > 0.0);

        self.tap_time_l[0] = self.srunit.ms_2_samples(80.3, m)  as i32;
        self.tap_time_l[1] = self.srunit.ms_2_samples(59.3, m)  as i32;
        self.tap_time_l[2] = self.srunit.ms_2_samples(97.7, m)  as i32;
        self.tap_time_l[3] = self.srunit.ms_2_samples(122.6, m) as i32;
        self.tap_time_r[0] = self.srunit.ms_2_samples(35.5, m)  as i32;
        self.tap_time_r[1] = self.srunit.ms_2_samples(101.6, m) as i32;
        self.tap_time_r[2] = self.srunit.ms_2_samples(73.9, m)  as i32;
        self.tap_time_r[3] = self.srunit.ms_2_samples(80.3, m)  as i32;

        self.input_allpass[0].set_len(self.srunit.ms_2_samples(4.76, m));
        self.input_allpass[1].set_len(self.srunit.ms_2_samples(6.81, m));
        self.input_allpass[2].set_len(self.srunit.ms_2_samples(10.13, m));
        self.input_allpass[3].set_len(self.srunit.ms_2_samples(16.72, m));

        self.allpass[[0,0]].set_len(self.srunit.ms_2_samples(38.2, m));
        self.allpass[[0,1]].set_len(self.srunit.ms_2_samples(53.4, m));
        self.delay[0].set_len(self.srunit.ms_2_samples(178.8, m) as i32);

        self.allpass[[1,0]].set_len(self.srunit.ms_2_samples(44.0, m));
        self.allpass[[1,1]].set_len(self.srunit.ms_2_samples(41.0, m));
        self.delay[1].set_len(self.srunit.ms_2_samples(126.5, m) as i32);

        self.allpass[[2,0]].set_len(self.srunit.ms_2_samples(48.3, m));
        self.allpass[[2,1]].set_len(self.srunit.ms_2_samples(60.5, m));
        self.delay[2].set_len(self.srunit.ms_2_samples(106.1, m) as i32);

        self.allpass[[3,0]].set_len(self.srunit.ms_2_samples(38.9, m));
        self.allpass[[3,1]].set_len(self.srunit.ms_2_samples(42.2, m));
        self.delay[3].set_len(self.srunit.ms_2_samples(139.4, m) as i32);
    }
}
