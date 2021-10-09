ix!();

use crate::{
    Lfo,
    LfoParam,
    LfoEnvState,
};

impl Lfo<'sr> {

    pub fn attack(&mut self) {

        if ! self.phase_initialized {
            self.init_phase_from_start_phase();
        }

        self.env_state = LfoEnvState::Delay;
        self.env_val   = 0.0;
        self.env_phase = 0.0;

        let start_phase = pvalf![self.params[LfoParam::StartPhase]];
        let shape       = LfoShape::try_from(pvali![self.params[LfoParam::Shape]] as usize).unwrap();
        let mode        = LfoMode::try_from(pvali![self.params[LfoParam::Trigmode]] as usize).unwrap();
        let delay       = pvalf![self.params[LfoParam::Delay]];
        let attack      = pvalf![self.params[LfoParam::Attack]];
        let hold        = pvalf![self.params[LfoParam::Hold]];
        let rate        = pvalf![self.params[LfoParam::Rate]];

        self.maybe_update_env_state_for_attack(delay,attack,hold);
        self.maybe_update_phase_and_step_for_attack(shape,mode,start_phase,rate);

        self.attack_shape(shape,start_phase);
    }
}
