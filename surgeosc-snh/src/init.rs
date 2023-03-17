crate::ix!();

pub const INITIAL_RATE: f64 = 0.05;

impl Init for SampleAndHoldOscillator {

    fn init(&mut self) {

        self.first_run = true;

        unsafe {
            self.blitter.osc_out_l = z128![];
            self.blitter.osc_out_r = z128![];
        }

        self.blitter.bufpos = 0;
        self.dc             = 0.0;

        self.l_pw.set_rate(INITIAL_RATE);
        self.l_shape.set_rate(INITIAL_RATE);
        self.l_sub.set_rate(INITIAL_RATE);
        self.l_sync.set_rate(INITIAL_RATE);

        self.blitter.n_unison = limit_range(
            self.pvali(SampleAndHoldOscillatorParam::UniCount), 
            1, MAX_UNISON as i32);

        macro_rules! clear_buffer {
            ($x:expr) => {
                for x in $x.iter_mut() { *x = 0.0; }
            }
        }
        clear_buffer![self.blitter.oscbuffer_l];
        clear_buffer![self.blitter.oscbuffer_r];
        clear_buffer![self.last_level];
        clear_buffer![self.last_level2];
        clear_buffer![self.elapsed_time];
    }
}
