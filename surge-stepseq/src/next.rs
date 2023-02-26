crate::ix!();

impl StepSequencer {

    pub fn next_step(&mut self) -> f32 {

        let step = self.steps[self.current_step];

        // Handle shuffle
        let mut rng = thread_rng();

        let shuffle_amount = self.shuffle * 0.5;

        let shuffle_offset = rng.gen_range(-shuffle_amount..shuffle_amount);

        let next_step = ((self.current_step + 1) % N_STEPSEQUENCER_STEPS) as f32
            + shuffle_offset
            + self.loop_start as f32;

        // Handle looping and wrapping around
        let mut wrapped_step = next_step;

        while wrapped_step > self.loop_end as f32 {
            wrapped_step -= (self.loop_end - self.loop_start + 1) as f32;
        }

        while wrapped_step < self.loop_start as f32 {
            wrapped_step += (self.loop_end - self.loop_start + 1) as f32;
        }

        // Handle trigmask
        if self.trigmask & (1 << wrapped_step as u64) == 0 {
            self.current_step = (self.current_step + 1) % N_STEPSEQUENCER_STEPS;
        } else {
            self.current_step = wrapped_step as usize;
        }

        step
    }
}
