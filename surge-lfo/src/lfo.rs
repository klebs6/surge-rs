crate::ix!();

enhanced_enum![
    LfoEnvState {
        Off,
        Delay,
        Attack,
        Hold,
        Decay,
        Release,
        Stuck,
    }
];

#[derive(Debug)]
pub struct Lfo {
    pub params:             LfoParamArray::<ParamRT::<LfoParam>>,
    pub output:             f64,
    pub stepsequencer:      StepSequencer,
    pub phase_initialized:  bool,
    pub env_val:            f32,
    pub env_state:          LfoEnvState,
    pub retrigger_feg:      bool,
    pub retrigger_aeg:      bool,
    pub phase:              f32,
    pub target:             f32,
    pub noise:              f32,
    pub noised1:            f32,
    pub env_phase:          f32,
    pub ratemult:           f32,
    pub env_releasestart:   f32,
    pub iout:               f32,
    pub wf_history:         [f32; 4],
    pub step:               isize,
    pub shuffle_id:         isize,
    pub sine:               QuadrOsc,
    pub time_unit:          TimeUnitHandle,
    pub tables:             TablesHandle,
    pub enabled:            bool,
}

name![Lfo,"LFO"];

#[inline] pub fn lfo_phaseincrement(samples: i32 , mut rate: f32 ) -> f32 
{
   rate = 1.0 - rate;
   samples as f32 * ENV_PHASEMULTI / (1.0 + LFO_RANGE * rate * rate * rate)
}
