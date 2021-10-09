ix!();

use crate::{
    Lfo,
    LfoParam,
    LfoEnvState,
};

impl Lfo<'sr> {

    pub fn release(&mut self) {

        let release_f = pvalf![
            self.params[LfoParam::Release]
        ];

        let release_f_max = pvalmaxf![
            self.params[LfoParam::Release]
        ];

        if release_f < release_f_max {
            self.env_state = LfoEnvState::Release;
            self.env_releasestart = self.env_val;
            self.env_phase = 0.0;
        }
    }

    pub fn reset(&mut self) {
        todo!();
    }
}
