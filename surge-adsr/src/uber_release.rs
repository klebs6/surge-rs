crate::ix!();

impl UberRelease for AdsrEnvelope {

    /// `AdsrEnvelope::uber_release`: This
    /// function puts the envelope into an "Uber
    /// Release" state, which sets the
    /// `scalestage` and `phase` fields and
    /// changes the `envstate` to
    /// `AdsrState::UberRelease`.
    ///
    fn uber_release(&mut self) 
    {
        //note, there was some other commented
        //logic here before the port
        self.scalestage = self.output;
        self.phase      = 1.0;
        self.envstate   = AdsrState::UberRelease;
    }
}
