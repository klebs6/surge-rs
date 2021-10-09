ix!();

use crate::{
    FM2Oscillator,
    //FM2OscillatorParam,
};

impl FM2Oscillator<'sr> {

    pub fn do_fm2_block(&mut self, k: usize, omega_clamped: f64, fm: bool) {

        self.rm1.process();
        self.rm2.process();

        self.out.l[k] = (self.phase + 
            self.rel_mod_depth1.v * self.rm1.r + 
            self.rel_mod_depth2.v * self.rm2.r + 
            self.lastoutput + 
            self.phase_offset.v) as f32;

        if fm {
            let master_osc: f64 = unsafe { *self.master_osc.add(k) as f64 };
            self.out.l[k] += (self.fm_depth.v * master_osc) as f32;
        }

        self.out.l[k] = self.out.l[k].sin();
        self.lastoutput = (self.out.l[k] as f64) * self.feedback_depth.v;

        self.phase += omega_clamped;

        self.rel_mod_depth1.process();
        self.rel_mod_depth2.process();
        self.feedback_depth.process();

        if fm {
            self.fm_depth.process();
        }

        self.phase_offset.process();
    }
}
