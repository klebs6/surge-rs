ix!();

use crate::*;

impl Lfo {

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

    pub fn update_phase_and_step_for_lfo_mode_keytrigger(phaseslider: f32) {
        self.phase = phaseslider;
        self.step = 0;
    }

    pub fn update_phase_and_step_for_lfo_mode_random() {

        self.phase = rand01();

        self.step = {
            let randi = rand01() as i32;
            let mask  = (N_STEPSEQUENCER_STEPS - 1);
            let step  = (randi % self.stepsequencer.loop_end) as usize;
            (step & mask) as isize
        };
    }

    pub fn update_phase_and_step_for_lfo_mode_free_run() {

        let x: f32 =  {
            let songpos = self.time_unit.songpos() as f32;
            let ratemod = 2.0_f32.powf( rate );

            phaseslider + 0.5 * songpos * ratemod
        };

        let (integral_part, _fractional_part) = split_float(x);

        let loop_start = self.stepsequencer.loop_start;
        let loop_end   = self.stepsequencer.loop_end;

        self.step = {
            let ipart               = integral_part as i32;
            let stepsequencer_delta = loop_end - loop_start;
            let delta_clamped       = max( 1, stepsequencer_delta);
            let offset              = loop_start;
            ( (ipart % delta_clamped ) + offset) as isize
        };
    }

    pub fn rezero_phase_and_step() {
        self.step  = 0;
        self.phase = 0.0;
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

            LfoMode::KeyTrigger => 
                update_lfo_mode_keytrigger(phaseslider),

            LfoMode::Random =>
                update_lfo_mode_random(),

            LfoMode::FreeRun =>
                update_lfo_mode_free_run(),

            _ => rezero_phase_and_step(),
        }
    }
}
