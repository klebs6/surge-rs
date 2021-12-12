ix!();

use crate::{
    SurgeSuperOscillator,
    SSOParam,
};

impl Init for SurgeSuperOscillator {

    fn init(&mut self) {

        self.first_run = true;

        let mode = self.get_character();

        self.set_coeff_by_character(mode);

        self.blitter.clear_all_oscout();

        self.blitter.bufpos = 0;
        self.dc = 0.0;

        self.set_rate_all(0.05);

        let unicount = self.pvali(SSOParam::UniCount);

        self.blitter.n_unison = limit_range( unicount, 1, MAX_UNISON as i32);

        self.blitter.prepare_unison(self.blitter.n_unison as usize);

        self.blitter.clear_buffers();

        self.clear_tracking();
    }
}
