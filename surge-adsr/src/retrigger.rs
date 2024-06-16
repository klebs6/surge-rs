crate::ix!();

pub trait Retrigger {
    fn retrigger(&mut self);
}

impl Retrigger for AdsrEnvelope {

    /// `AdsrEnvelope::retrigger`: This function
    /// retriggers the envelope by setting it to
    /// the "Attack" state if it is currently in
    /// a state less than "Release".
    ///
    fn retrigger(&mut self) {

        if self.envstate_is_before_release() {
            self.attack();
        }
    }
}
