ix!();

use crate::{
    FreqShiftParam,
    FreqShiftParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct FreqShift<'sr> {
    pub fr:            Align16<HalfRateFilterSSE>,
    pub fi:            Align16<HalfRateFilterSSE>,
    pub mix:           Align16<LipolPs>,
    pub ringout:       Ringout,
    pub params:        FreqShiftParamArrayRT,
    pub feedback:      LiPol<f32>,
    pub time:          Lag<f32>,
    pub shift_l:       Lag<f32>,
    pub shift_r:       Lag<f32>,
    pub inithadtempo:  bool,
    pub buffer:        A2d::<f32>,
    pub wpos:          i32,
    pub o1_l:          QuadrOsc,
    pub o2_l:          QuadrOsc,
    pub o1_r:          QuadrOsc,
    pub o2_r:          QuadrOsc,
    pub tables:        TablesHandle<'sr>,
    pub time_unit:     TimeUnitHandle<'sr>,
    pub tuner:         TunerHandle<'sr>,
    pub srunit:        SampleRateHandle<'sr>,
}

effect!       [FreqShift<'sr>,  FreqShiftParam];
has_timeunit! [FreqShift<'sr>,  FreqShiftParam];
no_op!        [FreqShift<'sr>,  ProcessOnlyControl];
name!         [FreqShift<'sr>,  "freqshift"];
