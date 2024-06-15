crate::ix!();

impl CheckIsIdle for AdsrEnvelope {

    /// `AdsrEnvelope::is_idle`: This function
    /// returns `true` if the envelope is
    /// currently in an "Idle" state and the
    /// `idlecount` is greater than 0, and `false`
    /// otherwise.
    ///
    fn is_idle(&self) -> bool 
    {
        self.envstate == AdsrState::Idle && self.idlecount > 0
    }
}
