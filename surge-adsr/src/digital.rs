crate::ix!();

impl AdsrEnvelope {

    pub fn process_block_digital(&mut self) {

        match self.envstate {
            AdsrState::Attack      => self.process_block_digital_attack(),
            AdsrState::Decay       => self.process_block_digital_decay(),
            AdsrState::Release     => self.process_block_digital_release(),
            AdsrState::UberRelease => self.process_block_digital_uberrelease(),
            AdsrState::Idle        => self.idlecount += 1,
            _ => {
                //sustain and idlewait not covered
            }
        }

        self.output = limit_range(self.output, 0.0, 1.0);
    }
}

