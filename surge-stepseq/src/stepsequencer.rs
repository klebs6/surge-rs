/*!
  | Here are a few more ideas to take your step
  | sequencer to the next level:
  |
  | 1. Adding the ability to save and load
  | sequences would allow users to create and
  | reuse their own custom sequences.
  |
  | 2. Implementing a "note mode" where each step
  | corresponds to a specific note value, allowing
  | users to create melodic sequences.
  |
  | 3. Adding a "chord mode" where users can
  | program chords across multiple steps.
  |
  | 4. Implementing a "probability mode" where the
  | likelihood of a step being triggered is based
  | on a probability distribution function
  | (e.g. normal distribution, Poisson
  | distribution, etc.).
  |
  | 5. Implementing a "sequence variation" mode
  | where the sequence is automatically mutated or
  | transformed over time according to
  | user-defined rules.
  |
  | 6. Adding the ability to chain multiple
  | sequences together, allowing for longer and
  | more complex patterns.
  |
  */

crate::ix!();

/// The StepSequencer struct represents a step
/// sequencer, which is an LFO shape that cycles
/// through a series of predefined steps.
///
#[derive(Debug,Clone)]
pub struct StepSequencer {

    /// An array of f32 values representing the
    /// steps in the sequence.
    ///
    pub(crate) steps:         [f32;        N_STEPSEQUENCER_STEPS],

    /// stores the probability of each step being
    /// triggered. 
    ///
    pub(crate) probabilities: [f32;        N_STEPSEQUENCER_STEPS],

    /// The index of the first step in the loop.
    ///
    pub(crate) loop_start:    usize,

    /// The index of the last step in the loop.
    ///
    pub(crate) loop_end:      usize,

    /// A value between 0 and 1 indicating the
    /// amount of shuffle to apply to the
    /// sequence.
    ///
    pub(crate) shuffle:       f32,

    /// A bitmask indicating which steps in the
    /// sequence should be triggered.
    ///
    pub(crate) trigmask:      u64,

    /// new field to store mute/unmute status
    ///
    pub(crate) mute_mask:     u64, 

    pub(crate) current_step:  usize,
}

/// This constant defines the number of steps in
/// a step sequencer, which is a musical tool that
/// allows you to program a sequence of notes or
/// other events. 
///
/// In this case, the step sequencer has 16 steps,
/// meaning that you can program up to 16 events
/// in a sequence.
///
pub const N_STEPSEQUENCER_STEPS: usize = 16;

impl Default for StepSequencer {

    fn default() -> Self {
        Self {
            steps:         [0.0;  N_STEPSEQUENCER_STEPS],
            probabilities: [1.0;  N_STEPSEQUENCER_STEPS],
            loop_start:    0,
            loop_end:      N_STEPSEQUENCER_STEPS - 1,
            shuffle:       0.0,
            trigmask:      0,
            mute_mask:     0,
            current_step:  0,
        }
    }
}

/// In this implementation, the struct fields are
/// private, and we've added getter and setter
/// methods for each field to allow access to the
/// struct's data. 
///
/// This makes the implementation more
/// encapsulated and allows us to modify the
/// implementation details without affecting other
/// parts of the code that use the
/// `StepSequencer`.
///
impl StepSequencer {

    pub fn new_from_sequence(steps: [f32; N_STEPSEQUENCER_STEPS]) -> Self {

        Self { 
            steps, 
            ..Default::default()
        }
    }

    pub fn set_step(&mut self, index: usize, value: f32) {
        if index < N_STEPSEQUENCER_STEPS {
            self.steps[index] = value;
        }
    }

    pub fn step(&self, index: usize) -> f32 {
        if index < N_STEPSEQUENCER_STEPS {
            self.steps[index]
        } else {
            0.0
        }
    }

    /// Sets the start and end points of the loop.
    ///
    pub fn set_loop(&mut self, start: usize, end: usize) {
        self.loop_start = start;
        self.loop_end = end;
    }

    pub fn set_loop_start(&mut self, value: usize) {
        self.loop_start = value;
    }

    pub fn loop_start(&self) -> usize {
        self.loop_start
    }

    pub fn set_loop_end(&mut self, value: usize) {
        self.loop_end = value;
    }

    pub fn loop_end(&self) -> usize {
        self.loop_end
    }

    /// Sets the amount of shuffle applied to the
    /// sequence. 
    ///
    /// Shuffle randomly offsets the next step by
    /// a small amount within a range of +/-
    /// `amount / 2`.
    ///
    pub fn set_shuffle(&mut self, value: f32) {
        self.shuffle = value;
    }

    pub fn shuffle(&self) -> f32 {
        self.shuffle
    }

    /// Sets a binary mask for triggering steps. 
    ///
    /// If a bit in the mask is set to 0, the step
    /// at that index will be skipped over when
    /// the sequencer advances.
    ///
    pub fn set_trigmask(&mut self, value: u64) {
        self.trigmask = value;
    }

    pub fn trigmask(&self) -> u64 {
        self.trigmask
    }

    /// Resets the sequencer to the beginning.
    ///
    pub fn reset(&mut self) {
        self.current_step = 0;
    }

    /// Returns the current step index.
    ///
    pub fn current_step(&self) -> usize {
        self.current_step
    }
}
