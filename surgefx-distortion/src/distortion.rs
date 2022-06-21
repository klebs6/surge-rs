crate::ix!();

pub const DISTORTION_OS_BITS:  usize = 2;
pub const DISTORTION_OS:       usize = 1 << DISTORTION_OS_BITS;

#[derive(Debug,Clone)]
pub struct Distortion {
    // feedback kan bli knepigt med sse-packed
    pub hr_a:           Align16<HalfRateFilterSSE>,
    pub hr_b:           Align16<HalfRateFilterSSE>,
    pub drive:          Align16<LipolPs>,
    pub outgain:        Align16<LipolPs>,
    pub ringout:        Ringout,
    pub params:         DistortionParamArrayRT,
    pub band1:          BiquadFilter,
    pub band2:          BiquadFilter,
    pub lp1:            BiquadFilter,
    pub lp2:            BiquadFilter,

    /// block_increment to keep track of 
    /// events not occurring every n blocks
    pub block_increment:i32, 
    pub left:           f32,
    pub right:          f32,
    pub tables:         TablesHandle,

    pub wetblock:       WetBlock2::<128>,
}

no_op! [Distortion, ProcessOnlyControl];
effect![Distortion,    DistortionParam];
name!  [Distortion,       "distortion"];
no_op! [Distortion,            Suspend];
