ix!();

use crate::{
    WTOscillator,
    WTOscillatorParam,
};

impl Init for WTOscillator {

    fn init(&mut self) { 

        self.first_run = true;
        unsafe {
            self.blitter.osc_out_l = z128![];
            self.blitter.osc_out_r = z128![];
        }
        self.blitter.bufpos = 0;

        let rate: f32 = 0.05;

        self.l_shape.set_rate(rate);
        self.l_clip.set_rate(rate);
        self.l_vskew.set_rate(rate);
        self.l_hskew.set_rate(rate);

        let unicount = self.pvali(WTOscillatorParam::UniCount);

        self.blitter.n_unison = limit_range_i(
            unicount, 1, MAX_UNISON as i32);

        if wt_flag![self,IsSample] {
            self.sampleloop = self.blitter.n_unison;
            self.blitter.n_unison = 1;
        }
        self.clear_last_level();
    }
}
