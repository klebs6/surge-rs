crate::ix!();

impl StepSequencer {

    /// Here's one way to implement swing on the
    /// StepSequencer:
    ///
    /// Here, `swing_amount` is a value between
    /// 0 and 1 that determines the amount of
    /// swing to apply. 
    ///
    /// A value of 0 means no swing, and a value
    /// of 1 means full swing, where every other
    /// step is delayed by the same amount. 
    ///
    /// The `swing_steps` variable calculates the
    /// amount of delay to apply to each even
    /// step. 
    ///
    /// Then, the loop iterates over each even
    /// step in the range from `loop_start` to
    /// `loop_end`, and adds the `swing_steps`
    /// value to the step's value. 
    ///
    /// The resulting value is clamped between
    /// 0 and 1 using the `max` and `min`
    /// functions.
    /// 
    /// Note that this implementation assumes that
    /// the StepSequencer has an even number of
    /// steps, and that `loop_start` and
    /// `loop_end` are both even numbers. 
    ///
    /// If these conditions are not met, the swing
    /// may not be evenly distributed across all
    /// steps.
    ///
    pub fn apply_swing(&mut self, swing_amount: f32) {

        let swing_steps = (swing_amount * (self.loop_end - self.loop_start + 1) as f32).round() as i32;

        // Apply swing to each even step starting from loop_start
        for i in (self.loop_start..=self.loop_end).step_by(2) {

            let new_val = self.steps[i as usize] + swing_steps as f32 / N_STEPSEQUENCER_STEPS as f32;

            self.steps[i as usize] = new_val.max(0.0).min(1.0);
        }
    }
}
