ix!();

use crate::{
    ReverbParam,
    ReverbParamRT,
    ReverbParamArray,
    ReverbPreset,
};

#[derive(Debug,Clone)]
pub struct Reverb<'sr> {
    pub delay_pan_l: Align16<A1d::<f32>>,
    pub delay_pan_r: Align16<A1d::<f32>>,
    pub delay_fb:    Align16<A1d::<f32>>,
    pub delay:       Align16<A1d::<f32>>,
    pub out_tap:     Align16<A1d::<f32>>,
    pub predelay:    Align16<A1d::<f32>>,
    pub delay_time:  Align16<A1d::<usize>>,
    pub mix:         Align16<LipolPs>,
    pub width:       Align16<LipolPs>,
    pub ringout:     Ringout,
    pub params:      ReverbParamArray::<ReverbParamRT>,
    pub delay_pos:   usize,
    pub modphase:    f64,
    pub preset:      ReverbPreset,
    pub lastf:       A1d::<f32>,
    pub band1:       BiquadFilter<'sr>,
    pub locut:       BiquadFilter<'sr>,
    pub hicut:       BiquadFilter<'sr>,
    pub b:           usize,
    pub time_unit:   TimeUnitHandle<'sr>,
    pub tables:      TablesHandle<'sr>,
    pub tuner:       TunerHandle<'sr>,
    pub srunit:      SampleRateHandle<'sr>,
}

no_op!        [Reverb<'sr>, ProcessOnlyControl];
effect!       [Reverb<'sr>,        ReverbParam];
has_timeunit! [Reverb<'sr>,        ReverbParam];
name!         [Reverb<'sr>,           "reverb"];
no_op!        [Reverb<'sr>,            Suspend];
no_update!    [Reverb<'sr>                    ];
