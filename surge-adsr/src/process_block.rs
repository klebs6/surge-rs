crate::ix!();

impl ProcessBlock for AdsrEnvelope {

    /// processes a block of samples for the ADSR
    /// envelope, either using the analog or
    /// digital implementation depending on the
    /// value of the `Mode` parameter.
    ///
    fn process_block(&mut self)
    {
        match self.is_analog() {
            true  => self.process_block_analog(),
            false => self.process_block_digital(),
        }
    }
}
