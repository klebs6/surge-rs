ix!();

use crate::{
    Lfo,
    LfoParam,
    N_STEPSEQUENCER_STEPS,
};

impl Lfo {

    pub fn update_for_phase_over_one(&mut self, shape: LfoShape) {

        self.phase -= 1.0;

        match shape {

            LfoShape::SampleAndHold => {
                self.iout = correlated_noise_o2mk2(self.target, self.noised1, 
                    pvalf![self.params[LfoParam::Deform]]
                );
            },

            LfoShape::Noise => {
                self.wf_history[3] = self.wf_history[2];
                self.wf_history[2] = self.wf_history[1];
                self.wf_history[1] = self.wf_history[0];
                self.wf_history[0] = correlated_noise_o2mk2(self.target, self.noised1, 
                    pvalf![self.params[LfoParam::Deform]]
                );
            },

            LfoShape::StepSequencer => {

                /*
                | You might thing we don't need
                | this and technically we don't but
                | I wanted to keep it here to retain
                | compatability with versions of
                | trigmask which were streamed in
                | older sessions
                */
                if (self.stepsequencer.trigmask & (1_u64 << self.step)) != 0 {
                    self.retrigger_feg = true;
                    self.retrigger_aeg = true;
                }

                if (self.stepsequencer.trigmask & (1_u64 << (16 + self.step))) != 0 {
                    self.retrigger_feg = true;
                }

                if (self.stepsequencer.trigmask & (1_u64 << (32 + self.step))) != 0 {
                    self.retrigger_aeg = true;
                }

                self.step += 1;
                self.shuffle_id = (self.shuffle_id + 1) & 1;

                if self.shuffle_id != 0 {
                    self.ratemult = 1.0 / minf(
                        0.01, 
                        1.0 - 0.5 * pvalf![self.params[LfoParam::StartPhase]]
                    );

                } else {
                    self.ratemult = 1.0 / (1.0 + 0.5 * 
                        pvalf![self.params[LfoParam::StartPhase]]
                    );
                }

                if self.step as i32 > self.stepsequencer.loop_end {
                    self.step = self.stepsequencer.loop_start as isize;
                }

                self.wf_history[3] = self.wf_history[2];
                self.wf_history[2] = self.wf_history[1];
                self.wf_history[1] = self.wf_history[0];
                self.wf_history[0] = self.stepsequencer.steps[(self.step as usize) & (N_STEPSEQUENCER_STEPS - 1)];
            },
            _ => {},
        }
    }
}
