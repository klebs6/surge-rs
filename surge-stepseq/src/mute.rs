crate::ix!();

/// With these methods, users can mute and unmute
/// individual steps or groups of steps in the
/// sequence as needed. 
///
/// For example, they could create a drum pattern
/// with a kick drum on every quarter note, but
/// mute the kicks on beats two and four to create
/// a more interesting rhythm.
///
impl StepSequencer {

    /// Mutes a specific step in the sequence.
    pub fn mute_step(&mut self, step: usize) {
        self.mute_mask |= 1 << step;
    }

    /// Unmutes a specific step in the sequence.
    pub fn unmute_step(&mut self, step: usize) {
        self.mute_mask &= !(1 << step);
    }

    /// Mutes all steps in the sequence.
    pub fn mute_all(&mut self) {
        self.mute_mask = u64::MAX >> (64 - N_STEPSEQUENCER_STEPS);
    }

    /// Unmutes all steps in the sequence.
    pub fn unmute_all(&mut self) {
        self.mute_mask = 0;
    }

    /// Mutes a group of steps in the sequence.
    ///
    /// The `mask` argument is a bit mask
    /// indicating which steps to mute. 
    ///
    /// The least significant bit corresponds to
    /// the first step in the sequence, and the
    /// most significant bit corresponds to the
    /// last step in the sequence.
    ///
    pub fn mute_group(&mut self, mask: u64) {
        self.mute_mask |= mask;
    }

    /// Unmutes a group of steps in the sequence.
    ///
    /// The `mask` argument is a bit mask
    /// indicating which steps to unmute. 
    ///
    /// The least significant bit corresponds to
    /// the first step in the sequence, and the
    /// most significant bit corresponds to the
    /// last step in the sequence.
    ///
    pub fn unmute_group(&mut self, mask: u64) {
        self.mute_mask &= !mask;
    }

    /// Checks whether a specific step in the
    /// sequence is muted.
    ///
    pub fn is_step_muted(&self, step: usize) -> bool {
        (self.mute_mask & (1 << step)) != 0
    }

    /// Gets the bit mask indicating which steps
    /// in the sequence are muted.
    ///
    pub fn get_mute_mask(&self) -> u64 {
        self.mute_mask
    }
}
