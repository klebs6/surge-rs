crate::ix!();

#[derive(Debug,Clone)]
#[name("reverb")]
pub struct Reverb {
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
    pub band1:       BiquadFilter,
    pub locut:       BiquadFilter,
    pub hicut:       BiquadFilter,
    pub b:           usize,
    pub time_unit:   TimeUnitHandle,
    pub tables:      TablesHandle,
    pub tuner:       TunerHandle,
    pub srunit:      SampleRateHandle,
}

no_op!        [Reverb, ProcessOnlyControl];
effect!       [Reverb,        ReverbParam];
has_timeunit! [Reverb,        ReverbParam];
no_op!        [Reverb,            Suspend];
no_update!    [Reverb                    ];
