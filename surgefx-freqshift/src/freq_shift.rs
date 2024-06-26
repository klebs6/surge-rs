crate::ix!();

///note, is this the right value?
pub const FREQSHIFT_MAX_DELAY_LENGTH: usize = 1 << 18; 

#[derive(Debug,Clone)]
#[name("freqshift")]
pub struct FreqShift {
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
    pub tables:        TablesHandle,
    pub time_unit:     TimeUnitHandle,
    pub tuner:         TunerHandle,
    pub srunit:        SampleRateHandle,
}

impl Reset for FreqShift {

    fn reset(&mut self) {
        todo!();
    }
}

effect!       [FreqShift,  FreqShiftParam];
has_timeunit! [FreqShift,  FreqShiftParam];
no_op!        [FreqShift,  ProcessOnlyControl];
