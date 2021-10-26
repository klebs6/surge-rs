ix!();

use crate::{
    AdsrEnvelope,
    AdsrState,
    AdsrParam,
};

impl AdsrEnvelope {

    pub fn process_block_digital_attack(&mut self) 
    {
        let lc_a: f32 = pvalf![self.params[AdsrParam::Attack]];

        self.phase += self.tables.envelope_rate_linear(lc_a) * tsyncratio![self,Attack];

        if self.phase >= 1.0
        {
            self.phase = 1.0;
            self.envstate = AdsrState::Decay;
            self.sustain = 
                pvalf![self.params[AdsrParam::Sustain]];
        }

        match pvali![self.params[AdsrParam::AttackShape]] 
        {
            0 => {
                self.output = self.phase.sqrt();
            },
            1 => {
                self.output = self.phase;
            },
            2 => {
                self.output = self.phase * self.phase;
            },
            _ => {panic!("logic bug")},
        }
    }
}
