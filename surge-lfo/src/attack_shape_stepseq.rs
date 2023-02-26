crate::ix!();

impl Lfo {

    #[inline] pub fn attack_shape_stepseq(&mut self, start_phase: f32) {

        // fire up the engines

        // This line sets the second element of the `wf_history` array to the current step of the
        // step sequencer, which is accessed using the `step` field of `self`. 
        //
        // The bitwise AND operation with `(N_STEPSEQUENCER_STEPS - 1)` is used to wrap around the
        // step sequencer when it reaches the end.
        //
        self.wf_history[1] = 
            self.stepsequencer.step(self.step & (N_STEPSEQUENCER_STEPS - 1));

        self.step += 1;

        // This block checks if the current step is greater than the `loop_end` field of the step
        // sequencer. 
        //
        // If it is, then the `step` field is set to the `loop_start` field, effectively looping
        // the step sequencer.
        //
        if self.step > self.stepsequencer.loop_end() {
            self.step = self.stepsequencer.loop_start();
        }

        // This line increments the `shuffle_id` field of `self`, which is used to determine
        // whether the rate multiplier should be calculated using the first or second formula in
        // the next block.
        //
        self.shuffle_id = (self.shuffle_id + 1) & 1;

        // This block calculates the rate multiplier based on the current `shuffle_id` and
        // `start_phase` values. 
        //
        // If `shuffle_id` is not zero, then the rate multiplier is calculated using the first
        // formula, which ensures that it is always greater than or equal to 0.01. 
        //
        // Otherwise, the rate multiplier is calculated using the second formula, which ensures
        // that it is always greater than zero.
        //
        if self.shuffle_id != 0 {

            self.ratemult = 1.0 / 
                maxf(
                    0.01, 
                    1.0 - 0.5 * start_phase 
                );

        } else {

            self.ratemult = 1.0 / (1.0 + 0.5 * start_phase);
        }

        // This line sets the first element of the `wf_history` array to the current step of the
        // step sequencer, which is accessed using the `step` field of `self`. 
        //
        // The bitwise AND operation with `(N_STEPSEQUENCER_STEPS - 1)` is used to wrap around the
        // step sequencer when it reaches the end.
        //
        self.wf_history[0] = 
            self.stepsequencer.step(self.step & (N_STEPSEQUENCER_STEPS - 1));
    }
}

