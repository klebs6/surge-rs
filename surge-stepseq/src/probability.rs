crate::ix!();

impl StepSequencer {

    /// set the probability for a single step
    ///
    pub fn set_probability(&mut self, step: usize, probability: f32) {
        if step < N_STEPSEQUENCER_STEPS {
            self.probabilities[step] = probability;
        }
    }

    /// set probabilities for all steps.
    ///
    pub fn set_probabilities(&mut self, probabilities: &[f32]) {

        let len = probabilities.len().min(N_STEPSEQUENCER_STEPS);

        self.probabilities[..len].copy_from_slice(&probabilities[..len]);
    }

    /// retrieve the probability for a single step
    ///
    pub fn get_probability(&self, step: usize) -> f32 {

        if step < N_STEPSEQUENCER_STEPS {
            self.probabilities[step]
        } else {
            0.0
        }
    }

    /// retrieve probabilities for all steps.
    ///
    pub fn get_probabilities(&self) -> &[f32] {
        &self.probabilities[..]
    }

    /// The `randomize_probabilities` function
    /// generates random probabilities for each
    /// step,
    ///
    pub fn randomize_probabilities(&mut self) {

        let mut rng = thread_rng();

        for i in 0..N_STEPSEQUENCER_STEPS {
            self.probabilities[i] = rng.gen_range(0.0..1.0);
        }
    }

    /// randomly mutates the probabilities with
    /// a given probability.
    ///
    pub fn mutate_probabilities(&mut self, amount: f32) {

        let mut rng = thread_rng();

        for i in 0..N_STEPSEQUENCER_STEPS {

            if rng.gen_bool(amount as f64) {
                self.probabilities[i] = rng.gen_range(0.0..1.0);
            }
        }
    }

    /// Finally, the `trigger_probability`
    /// function is used to determine whether
    /// a step should be triggered based on its
    /// probability.
    /// 
    pub fn trigger_probability(&mut self, step: usize) -> bool {

        let probability = self.get_probability(step);

        let mut rng = thread_rng();

        rng.gen_bool(probability as f64)
    }
}
