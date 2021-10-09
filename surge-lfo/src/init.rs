ix!();

use crate::{
    Lfo,
    LfoEnvState,
    LfoParam,
    StepSequencer,
};

impl Lfo<'sr> {

    pub fn new( 
        //time_unit: &'timeunit TimeUnitHandle<'timeunit>,
        //tables: &'sr TablesHandle<'sr>) -> Self 
        time_unit: TimeUnitHandle<'sr>,
        tables:    TablesHandle<'sr>) -> Self 
    {
        Self {
            params:             LfoParam::new_runtime(),
            output:             0.0,
            stepsequencer:      StepSequencer::default(),
            phase_initialized:  false,
            env_val:            0.0,
            env_state:          LfoEnvState::Delay,
            retrigger_feg:      false,
            retrigger_aeg:      false,
            phase:              0.0,
            target:             0.0,
            noise:              0.0,
            noised1:            0.0,
            env_phase:          0.0,
            ratemult:           1.0,
            env_releasestart:   0.0,
            iout:               0.0,
            wf_history:         [0.0; 4],
            step:               0,
            shuffle_id:         0,
            sine:               QuadrOsc::new(),
            time_unit:          time_unit.clone(),
            tables:             tables.clone(),
            enabled:            true,
        }
    }

    #[inline] pub fn init_phase_from_start_phase(&mut self) {
        self.phase = pvalf![self.params[LfoParam::StartPhase]];
        self.phase_initialized = true;
        while self.phase < 0.0 { self.phase += 1.0 ; }
        while self.phase > 1.0 { self.phase -= 1.0 ; }
    }
}
