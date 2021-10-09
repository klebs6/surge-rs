ix!();

use crate::{
    DistortionParam,
    DistortionParamArrayRT,
};

pub const DISTORTION_OS_BITS:  usize = 2;
pub const DISTORTION_OS:       usize = 1 << DISTORTION_OS_BITS;

#[derive(Debug,Clone)]
pub struct Distortion<'sr> {
    // feedback kan bli knepigt med sse-packed
    pub hr_a:           Align16<HalfRateFilterSSE>,
    pub hr_b:           Align16<HalfRateFilterSSE>,
    pub drive:          Align16<LipolPs>,
    pub outgain:        Align16<LipolPs>,
    pub ringout:        Ringout,
    pub params:         DistortionParamArrayRT,
    pub band1:          BiquadFilter<'sr>,
    pub band2:          BiquadFilter<'sr>,
    pub lp1:            BiquadFilter<'sr>,
    pub lp2:            BiquadFilter<'sr>,

    /// block_increment to keep track of 
    /// events not occurring every n blocks
    pub bi:             i32, 
    pub left:           f32,
    pub right:          f32,
    pub tables:         TablesHandle<'sr>,
}

no_op! [Distortion<'sr>, ProcessOnlyControl];
effect![Distortion<'sr>,    DistortionParam];
name!  [Distortion<'sr>,       "distortion"];
no_op! [Distortion<'sr>,            Suspend];
