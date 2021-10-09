ix!();

use crate::{
    AdsrEnvelope,
    AdsrParam,
    AdsrState,
};

impl AdsrEnvelope<'sr> {

    pub fn process_block_digital_uberrelease(&mut self) {

        self.phase -= self.tables.envelope_rate_linear(-6.5);

        self.output = self.phase;

        for _i in 0..pvali![self.params[AdsrParam::ReleaseShape]] {
            self.output *= self.phase;
        }

        if self.phase < 0.0
        {
            self.envstate = AdsrState::Idle;
            self.output = 0.0;
        }

        self.output *= self.scalestage;
    }
}
