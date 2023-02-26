crate::ix!();

impl Lfo {

    /// Sets the phase for the next processing
    /// block based on the tempo sync ratio and
    /// LFO rate.
    ///
    /// # Arguments
    ///
    /// * `temposyncratio`: A `f32` representing
    /// the tempo sync ratio for the LFO.
    ///
    pub fn set_phase_for_process(&mut self, temposyncratio: f32) 
    {
        let rate = self.get_rate(temposyncratio);

        self.phase += rate * self.ratemult;
    }

    /// Resets the retriggers for the LFO's AEG
    /// and FEG.
    ///
    pub fn zero_retriggers(&mut self) {
        self.retrigger_feg = false;
        self.retrigger_aeg = false;
    }
}
