ix!();

use crate::{
    Lfo,
    LfoParam,
    LfoEnvState,
    N_STEPSEQUENCER_STEPS,
};

impl Lfo<'sr> {

    pub fn maybe_update_env_state_for_attack(
        &mut self, 
        delay: f32, 
        attack: f32, 
        hold: f32) 
    {
        if (delay - pvalminf![self.params[LfoParam::Delay]]).abs() < f32::EPSILON {

            self.env_state = LfoEnvState::Attack;

            if (attack - pvalminf![self.params[LfoParam::Attack]]).abs() < f32::EPSILON
            {
                self.env_state = LfoEnvState::Hold;
                self.env_val   = 1.0;

                if (hold - pvalminf![self.params[LfoParam::Hold]]).abs() < f32::EPSILON  
                {
                    self.env_state = LfoEnvState::Decay;
                }
            }
        }
    }

    pub fn maybe_update_phase_and_step_for_attack(
        &mut self,
        lfo_shape:   LfoShape,
        lfo_mode:    LfoMode,
        start_phase: f32,
        rate:        f32)
    {
        let phaseslider = self.get_phaseslider(lfo_shape, start_phase);

        match lfo_mode {

            LfoMode::KeyTrigger => {
                self.phase = phaseslider;
                self.step = 0;
            },

            LfoMode::Random => {
                self.phase = rand01();
                self.step = ((((rand01() as i32) % self.stepsequencer.loop_end) as usize) & (N_STEPSEQUENCER_STEPS - 1)) as isize;
            },

            LfoMode::FreeRun => {

                let x: f32 = 
                    phaseslider + 
                    0.5 * 
                    (self.time_unit.songpos() as f32) * 
                    2.0_f32.powf( rate );

                let (integral_part, _fractional_part) = split_float(x);

                self.step = (
                    ((integral_part as i32) % 
                     std::cmp::max(
                         1, 
                         self.stepsequencer.loop_end - self.stepsequencer.loop_start
                     )
                    ) + self.stepsequencer.loop_start) as isize;
            },

            /*
            _ => {
                self.step = 0;
                self.phase = 0.0;
            },
            */
        }
    }
}
