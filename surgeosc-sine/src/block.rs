ix!();

use crate::{
    SineWaveOscillator,
    SineWaveOscillatorParam,
};

impl SineWaveOscillator<'sr> {

    pub fn do_sine_block(&mut self, k: usize, omega: f64, fm: bool) {

        let wf_mode: i32 = self.pvali(SineWaveOscillatorParam::Shape);

        let master_osc: f64 = unsafe { 
            *self.master_osc.add(k) as f64 
        };

        // Replicate FM2 exactly
        let mut p: f32 = 
            (self.phase + self.lastvalue as f64) as f32; 

        if fm {
            p += (self.fm_depth.v * master_osc) as f32;
        }

        self.out.l[k] = 
            Self::value_from_sin_and_cos( p.sin(), p.cos(), wf_mode );

        self.phase += omega;
        self.lastvalue = self.out.l[k] * (self.feedback.v as f32);
        self.fm_depth.process();
        self.feedback.process();
    }
}
