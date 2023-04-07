crate::ix!();

/// Here's an implementation of random sequence
/// generation and mutation for the StepSequencer:
///
impl StepSequencer {

    /// Generates a random sequence with values
    /// between `min` and `max` for each step. 
    ///
    pub fn random_sequence(min: f32, max: f32) -> StepSequencer {

        let rng = thread_rng();

        let seq: [f32; N_STEPSEQUENCER_STEPS] = rng
            .sample_iter(distributions::Uniform::new(min, max))
            .take(N_STEPSEQUENCER_STEPS)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();

        StepSequencer::new_from_sequence(seq)
    }

    /// Mutates an existing sequence by adding
    /// a random value to each step with
    /// a probability of `mutation_rate`. 
    ///
    /// The amount of mutation is determined by
    /// `mutation_range`, which specifies the
    /// maximum amount by which a step can be
    /// mutated.
    ///
    pub fn mutate_sequence(&mut self, mutation_rate: f32, mutation_range: f32) {

        let mut rng = thread_rng();

        for i in 0..N_STEPSEQUENCER_STEPS {

            if rng.gen::<f32>() < mutation_rate {
                self.steps[i] += rng.gen_range(-mutation_range..mutation_range);
                self.steps[i] = self.steps[i].max(0.0).min(1.0);
            }
        }
    }
}
