crate::ix!();

impl Initialize for AdsrEnvelope {

    fn init(&mut self) -> Result<(),SurgeError> 
    {
        self.set_envstate(AdsrState::Attack);
        self.clear_phase();
        self.clear_output();
        self.clear_idlecount();
        self.set_scalestage(1.0);
        self.reset_analog_state_machine();
        Ok(())
    }
}
