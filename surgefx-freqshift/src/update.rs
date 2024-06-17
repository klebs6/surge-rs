crate::ix!();

impl Update for FreqShift {

    fn update(&mut self) {

        self.feedback.new_value(amp_to_linear(self.pvalf(FreqShiftParam::Feedback)));

        self.time.new_value(
            (self.maybe_temposyncratio_inv(FreqShiftParam::Delay))
            * self.srunit.samplerate() 
            * (self.tuner.n2p::<f32,true>( 
                    12.0_f32 
                    * (self.pvalf(FreqShiftParam::Delay))
            )) 
            - (FIR_OFFSET as f32)
        );

        let mix = pvalf![self.params[FreqShiftParam::Mix]];

        self.mix.set_target_smoothed(mix);

        let shift: f64 = 
            (self.pvalf(FreqShiftParam::Shift) as f64) *
            match self.params[FreqShiftParam::Shift].extend_range()  { true => 1000.0, false => 10.0 } ;

        let mut omega: f64 = shift * PI * 2.0 * self.srunit.dsamplerate_inv();

        self.o1_l.set_rate(PI * 0.5 - mind(0.0, omega));
        self.o2_l.set_rate(PI * 0.5 + maxd(0.0, omega));

        // phase lock oscillators
        if (pvalf![self.params[FreqShiftParam::RMult]] - 1.0).abs() < f32::EPSILON 
        {
            let a: f64 = 0.01;
            self.o1_r.r = a * self.o1_l.r + (1.0 - a) * self.o1_r.r;
            self.o1_r.i = a * self.o1_l.i + (1.0 - a) * self.o1_r.i;
            self.o2_r.r = a * self.o2_l.r + (1.0 - a) * self.o2_r.r;
            self.o2_r.i = a * self.o2_l.i + (1.0 - a) * self.o2_r.i;

        } else {
            omega *= pvalf![self.params[FreqShiftParam::RMult]] as f64;
            omega *= pvalf![self.params[FreqShiftParam::RMult]] as f64;
        }

        self.o1_r.set_rate(PI * 0.5 - mind(0.0, omega));
        self.o2_r.set_rate(PI * 0.5 + maxd(0.0, omega));

        let maxfb: f32 = maxd(db96![], self.feedback.v as f64) as f32;

        if maxfb < 1.0 {

            let f: f32 = BLOCK_SIZE_INV * self.time.v * ((1.0 + (db96![] as f32).log(maxfb)) as f32);

            self.ringout = Ringout::blocks(f as NumberOfBlocks);

        } else {

            self.ringout = Ringout::blocks(0);
        }
    }
}
