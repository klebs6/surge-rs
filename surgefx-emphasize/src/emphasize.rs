ix!();

use crate::{
    EmphasizeParam,
    EmphasizeParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Emphasize<'sr> {
    pub ringout:           Ringout,
    pub params:            EmphasizeParamArrayRT,
    pub pre:               Align16<HalfRateFilterSSE>,
    pub post:              Align16<HalfRateFilterSSE>,
    pub ty:                Align16<LipolPs>,
    pub outgain:           Align16<LipolPs>,
    pub eq:                BiquadFilter<'sr>,

    ///(to keep track of events 
    ///not occurring every n blocks)
    pub block_increment:   i32,
    pub left:              f32,
    pub right:             f32,
}

no_op!     [Emphasize<'sr>, ProcessOnlyControl];
effect!    [Emphasize<'sr>,     EmphasizeParam];
name!      [Emphasize<'sr>,        "emphasize"];
no_op!     [Emphasize<'sr>,            Suspend];
no_update! [Emphasize<'sr>                    ];
