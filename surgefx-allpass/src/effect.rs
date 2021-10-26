ix!();

use crate::{
    Allpass,
    OnePoleFilter,
    AllpassDelay,
    AllpassPreDelay,
    AllpassReverbParam,
    AllpassReverbParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct AllpassVerb {
    pub mix                 : Align16<LipolPs>,
    pub width               : Align16<LipolPs>,
    pub ringout             : Ringout,
    pub params              : AllpassReverbParamArrayRT,
    pub input_allpass       : A1d::<Allpass>,
    pub allpass             : A2d::<Allpass>,
    pub hf_damper           : A1d::<OnePoleFilter>,
    pub lf_damper           : A1d::<OnePoleFilter>,
    pub delay               : A1d::<AllpassDelay>,
    pub predelay            : AllpassPreDelay,
    pub tap_time_l          : A1d::<i32>,
    pub tap_time_r          : A1d::<i32>,
    pub tap_gain_l          : A1d::<f32>,
    pub tap_gain_r          : A1d::<f32>,
    pub state               : f32,
    pub decay_multiply      : LiPol<f32>,
    pub diffusion           : LiPol<f32>,
    pub buildup             : LiPol<f32>,
    pub hf_damp_coefficient : LiPol<f32>,
    pub lf_damp_coefficient : LiPol<f32>,
    pub modulation          : LiPol<f32>,
    pub lfo                 : QuadrOsc,
    pub last_decay_time     : f32,
    pub srunit              : SampleRateHandle,
}

name!             [AllpassVerb, "reverb2"];
effect!           [AllpassVerb, AllpassReverbParam];
update_on_init!   [AllpassVerb];
default_default!  [AllpassVerb];
no_op!            [AllpassVerb, Suspend];
no_op!            [AllpassVerb, ProcessOnlyControl];

