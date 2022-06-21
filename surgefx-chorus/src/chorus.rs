crate::ix!();

#[derive(Debug,Clone)]
pub struct Chorus {

    pub feedback:     Align16<Box<LipolPs>>,
    pub mix:          Align16<Box<LipolPs>>,
    pub width:        Align16<Box<LipolPs>>,
    pub voicepan_l4:  Align16<A1d::<__m128>>,
    pub voicepan_r4:  Align16<A1d::<__m128>>,

    ///includes padding so we can use 
    ///SSE interpolation without wrapping
    pub buffer:       Align16<A1d::<f32>>,

    pub ringout:      Ringout,
    pub params:       ChorusParamArrayRT,
    pub time:         A1d::<Lag::<f32>>,
    pub voicepan:     A2d::<f32>,
    pub envf:         f32,
    pub wpos:         i32,
    pub lp:           BiquadFilter,
    pub hp:           BiquadFilter,
    pub lfophase:     A1d::<f64>,
    pub tables:       TablesHandle,
    pub tuner:        TunerHandle,
    pub time_unit:    TimeUnitHandle,
    pub srunit:       SampleRateHandle,
}

no_op!        [Chorus, ProcessOnlyControl];
effect!       [Chorus,        ChorusParam];
has_timeunit! [Chorus,        ChorusParam];
no_op!        [Chorus,            Suspend];
name!         [Chorus,           "chorus"];
