ix!();

use crate::{
    FMOscillator,
};

impl FMOscillator<'sr> {

    #[inline] pub fn do_fm_block(&mut self, k: usize, omega: f64, fm: bool) {

        let master_osc: f64 = unsafe { 
            *self.master_osc.add(k) as f64 
        };

        self.rm1.process();
        self.rm2.process();
        self.am.process();

        self.out.l[k] = 
            (self.phase + 
             self.rel_mod_depth1.v * self.rm1.r + 
             self.rel_mod_depth2.v * self.rm2.r + 
             self.abs_mod_depth.v * self.am.r +
             self.lastoutput) as f32;

        if fm {
            self.out.l[k] += (self.fm_depth.v * master_osc) as f32;
        }

        self.out.l[k] = self.out.l[k].sin();

        self.lastoutput = (self.out.l[k] * (self.feedback_depth.v as f32)) as f64;
        self.phase     += omega;

        self.rel_mod_depth1.process();
        self.rel_mod_depth2.process();
        self.abs_mod_depth.process();

        if fm {
            self.fm_depth.process();
        }
        self.feedback_depth.process();
    }
}
