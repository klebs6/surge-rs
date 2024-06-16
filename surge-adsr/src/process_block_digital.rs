crate::ix!();

pub trait ProcessBlockDigital {

    fn process_block_digital(&mut self);
}

impl ProcessBlockDigital for AdsrEnvelope {

    fn process_block_digital(&mut self) {

        match self.get_envstate() {
            AdsrState::Attack      => self.digital_attack(),
            AdsrState::Decay       => self.digital_decay(),
            AdsrState::Release     => self.digital_release(),
            AdsrState::UberRelease => self.digital_uberrelease(),
            AdsrState::Idle        => self.increment_idlecount(),
            _ => {
                //sustain and idlewait not covered
            }
        }

        self.clamp01();
    }
}
