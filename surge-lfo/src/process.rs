ix!();

use crate::{
    Lfo,
    LfoParam,
    LfoEnvState,
};

impl LfoProcess for Lfo {

    fn process(&mut self) {

        if ! self.phase_initialized {
            self.init_phase_from_start_phase();
        }

        self.zero_retriggers();

        let  shape = self.get_shape();

        let temposyncratio = self.time_unit.temposyncratio();

        self.set_phase_for_process(temposyncratio);

        if self.env_state != LfoEnvState::Stuck {
            self.update_envelope_for_process(temposyncratio);
        }

        if self.phase > 1.0 {
            self.update_for_phase_over_one(shape);
        }

        self.process_shape(shape);

        let mut io2: f32 = self.iout;

        let unipolar  = pvalb![self.params[LfoParam::Unipolar]];
        let magnitude = pvalf![self.params[LfoParam::Magnitude]];

        if unipolar && (shape != LfoShape::StepSequencer) {
                io2 = 0.5 + 0.5 * io2;
        }

        self.output = (self.env_val * magnitude * io2) as f64;
    }
}
