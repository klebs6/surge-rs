ix!();

use crate::{
    DualDelayParam,
    DualDelayParamArrayRT,
};

pub const DUAL_DELAY_MAX_DELAY_LENGTH: usize = 1 << 18;

#[derive(Debug,Clone)]
pub struct DualDelay<'sr> {
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
    pub lp:              BiquadFilter<'sr>,
    pub hp:              BiquadFilter<'sr>,
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
    pub time_unit:       TimeUnitHandle<'sr>,
    pub tuner:           TunerHandle<'sr>,
    pub tables:          TablesHandle<'sr>,
    pub srunit:          SampleRateHandle<'sr>,
}

name!        [DualDelay<'sr>, "dualdelay"   ]; 
effect!      [DualDelay<'sr>, DualDelayParam];
no_op!       [DualDelay<'sr>, Suspend       ];
has_timeunit![DualDelay<'sr>, DualDelayParam];
