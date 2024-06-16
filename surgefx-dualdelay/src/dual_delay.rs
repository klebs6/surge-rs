crate::ix!();

pub const DUAL_DELAY_MAX_DELAY_LENGTH: usize = 1 << 18;

#[derive(Debug,Clone)]
#[name("dualdelay")]
pub struct DualDelay {
    pub feedback:        Align16<LipolPs>,
    pub crossfeed:       Align16<LipolPs>,
    pub aligpan:         Align16<LipolPs>,
    pub pan:             Align16<LipolPs>,
    pub mix:             Align16<LipolPs>,
    pub width:           Align16<LipolPs>,
    pub buffer:          Align16<A2d::<f32>>,
    pub ringout:         Ringout,
    pub params:          DualDelayParamArrayRT,
    pub time_l:          Lag::<f32>,
    pub time_r:          Lag::<f32>,
    pub inithadtempo:    bool,
    pub envf:            f32,
    pub wpos:            i32,
    pub lp:              BiquadFilter,
    pub hp:              BiquadFilter,
    pub lfophase:        f64,
    pub lfo_val:         f32,
    pub lfo_direction:   bool,

    //--------------------------------------------
    //are we sure we want to keep these here?
    //i guess it is the best place for them
    pub scratch_left:    ScratchChannel::<f32>,
    pub scratch_right:   ScratchChannel::<f32>,
    pub wetblock:        WetBlock2::<BLOCK_SIZE>,

    //--------------------------------------------
    pub time_unit:       TimeUnitHandle,
    pub tuner:           TunerHandle,
    pub tables:          TablesHandle,
    pub srunit:          SampleRateHandle,
}

effect!      [DualDelay, DualDelayParam];
no_op!       [DualDelay, Suspend       ];
has_timeunit![DualDelay, DualDelayParam];
