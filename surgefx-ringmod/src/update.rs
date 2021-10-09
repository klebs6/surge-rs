ix!();

use crate::RingModulator;

impl RingModulator<'sr> {
    pub fn update_unison_settings(&mut self, uni: i32) {
        self.last_unison = uni;

        match uni {
            1 => {
                self.detune_offset[0] = 0.0;
                self.pan_l[0]          = 1.0;
                self.pan_r[0]          = 1.0;
                self.phase[0]         = 0.0;
            },
            _ => {
                let detune_bias: f32 = 2.0 / (uni as f32 - 1.0);
                for u in 0..uni {
                    let idx = u as usize;
                    let u   = u as f32;
                    self.phase[idx]         = u * 1.0 / ( uni as f32 );
                    self.detune_offset[idx] = -1.0 + detune_bias * u;
                    self.pan_l[idx]          = u / (uni as f32 - 1.0 );
                    self.pan_r[idx]          = (uni as f32 - 1.0 - u ) / (uni as f32 - 1.0);
                }
            },
        }
    }
}
