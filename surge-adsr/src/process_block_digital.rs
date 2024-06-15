crate::ix!();

impl ProcessBlockDigital for AdsrEnvelope {

    fn process_block_digital(&mut self) {

        match self.envstate {
            AdsrState::Attack      => self.digital_attack(),
            AdsrState::Decay       => self.digital_decay(),
            AdsrState::Release     => self.digital_release(),
            AdsrState::UberRelease => self.digital_uberrelease(),
            AdsrState::Idle        => self.idlecount += 1,
            _ => {
                //sustain and idlewait not covered
            }
        }

        self.clamp01();
    }
}
