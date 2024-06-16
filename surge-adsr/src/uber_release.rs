crate::ix!();

pub trait UberRelease {
    fn uber_release(&mut self);
}

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
        let output = self.get_output();

        //note, there was some other commented
        //logic here before the port
        self.set_scalestage(output as f32);
        self.set_phase(1.0);
        self.set_envstate(AdsrState::UberRelease);
    }
}
