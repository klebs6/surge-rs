ix!();
use crate::{
    ChorusParam,
    ChorusParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Chorus<'sr> {

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
    pub lp:           BiquadFilter<'sr>,
    pub hp:           BiquadFilter<'sr>,
    pub lfophase:     A1d::<f64>,
    pub tables:       TablesHandle<'sr>,
    pub tuner:        TunerHandle<'sr>,
    pub time_unit:    TimeUnitHandle<'sr>,
    pub srunit:       SampleRateHandle<'sr>,
}

no_op!        [Chorus<'sr>, ProcessOnlyControl];
effect!       [Chorus<'sr>,        ChorusParam];
has_timeunit! [Chorus<'sr>,        ChorusParam];
no_op!        [Chorus<'sr>,            Suspend];
name!         [Chorus<'sr>,           "chorus"];
