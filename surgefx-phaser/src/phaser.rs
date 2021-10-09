ix!();

use crate::{
    PhaserParam,
    PhaserParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Phaser<'sr> {

    pub mix:         Align16<LipolPs>,
    pub l:           Align16<A1d::<f32>>,
    pub r:           Align16<A1d::<f32>>,

    pub ringout:     Ringout,
    pub params:      PhaserParamArrayRT,

    pub feedback:    LiPol<f32>,
    pub d_l:         f32,
    pub d_r:         f32,
    pub biquad:      A1d::<BiquadFilter<'sr>>,
    pub lfophase:    f32,

    /// block increment (to keep track of 
    /// events not occurring every n blocks)
    pub bi:          i32, 

    pub tables:      TablesHandle<'sr>,
    pub time_unit:   TimeUnitHandle<'sr>,
}

name!         [Phaser<'sr>, "phaser"]; 
effect!       [Phaser<'sr>, PhaserParam];
no_op!        [Phaser<'sr>, Suspend];
has_timeunit! [Phaser<'sr>, PhaserParam];
