ix!();

use crate::*;

impl Lfo {

    pub fn set_phase_for_process(&mut self, temposyncratio: f32) 
    {
        let rate = self.get_rate(temposyncratio);

        self.phase += rate * self.ratemult;
    }

    pub fn zero_retriggers(&mut self) {
        self.retrigger_feg = false;
        self.retrigger_aeg = false;
    }
}
