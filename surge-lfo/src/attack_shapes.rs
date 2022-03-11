ix!();

use crate::{
    Lfo,
    LfoParam,
    N_STEPSEQUENCER_STEPS,
};

impl Lfo {

    #[inline] pub fn attack_shape(&mut self, shape: LfoShape, start_phase: f32) {
        match shape {
            LfoShape::Sine          => self.attack_shape_sine(),
            LfoShape::Tri           => self.attack_shape_tri(),
            LfoShape::Square        => self.attack_shape_square(),
            LfoShape::Ramp          => self.attack_shape_ramp(),
            LfoShape::Noise         => self.attack_shape_noise(),
            LfoShape::SampleAndHold => self.attack_shape_snh(),
            LfoShape::Envelope      => self.attack_shape_envelope(),
            LfoShape::StepSequencer => self.attack_shape_stepseq(start_phase),
        }
    }

    #[inline] pub fn attack_shape_sine(&mut self) {

        if pvalb![self.params[LfoParam::Unipolar]] { 
            self.phase += 0.75; 
        }

        if self.phase > 1.0 { 
            self.phase -= 1.0; 
        }
    }

    #[inline] pub fn attack_shape_tri(&mut self) {

        if ! pvalb![self.params[LfoParam::Unipolar]] {

            self.phase += 0.25;

            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
        }
    }

    #[inline] pub fn attack_shape_square(&mut self)   { /* no-op TODO: is it? */ }
    #[inline] pub fn attack_shape_ramp(&mut self)     { /* no-op TODO: is it? */ }
    #[inline] pub fn attack_shape_envelope(&mut self) { /* no-op TODO: is it? */ }

    #[inline] pub fn attack_shape_noise(&mut self) 
    {
        self.noise   = 0.0;
        self.noised1 = 0.0;
        self.target  = 0.0;

        self.wf_history[3] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.wf_history[2] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.wf_history[1] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.wf_history[0] = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        ) * self.phase;

        self.phase = 0.0;
    }

    #[inline] pub fn attack_shape_snh(&mut self) 
    {
        self.noise   = 0.0;
        self.noised1 = 0.0;
        self.target  = 0.0;

        self.iout    = correlated_noise_o2mk2(
            self.target, 
            self.noised1, 
            pvalf![self.params[LfoParam::Deform]]
        );
    }

    #[inline] pub fn attack_shape_stepseq(&mut self, start_phase: f32) {

        // fire up the engines
        self.wf_history[1] = 
            self.stepsequencer.steps[(self.step as usize & (N_STEPSEQUENCER_STEPS - 1)) as usize];

        self.step += 1;

        if self.step > self.stepsequencer.loop_end as isize {
            self.step = self.stepsequencer.loop_start as isize;
        }

        self.shuffle_id = (self.shuffle_id + 1) & 1;

        if self.shuffle_id != 0 {

            self.ratemult = 1.0 / 
                maxf(
                    0.01, 
                    1.0 - 0.5 * start_phase 
                );

        } else {

            self.ratemult = 1.0 / (1.0 + 0.5 * start_phase);
        }

        self.wf_history[0] = 
            self.stepsequencer.steps[ (self.step as usize & (N_STEPSEQUENCER_STEPS - 1)) as usize ];
    }
}
