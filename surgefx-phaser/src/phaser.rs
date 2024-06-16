crate::ix!();

#[derive(Debug,Clone)]
#[name("phaser")]
pub struct Phaser {

    pub mix:         Align16<LipolPs>,
    pub l:           Align16<A1d::<f32>>,
    pub r:           Align16<A1d::<f32>>,

    pub ringout:     Ringout,
    pub params:      PhaserParamArrayRT,

    pub feedback:    LiPol<f32>,
    pub d_l:         f32,
    pub d_r:         f32,
    pub biquad:      A1d::<BiquadFilter>,
    pub lfophase:    f32,

    /// block increment (to keep track of 
    /// events not occurring every n blocks)
    pub bi:          i32, 

    pub tables:      TablesHandle,
    pub time_unit:   TimeUnitHandle,
}

effect!       [Phaser, PhaserParam];
no_op!        [Phaser, Suspend];
has_timeunit! [Phaser, PhaserParam];
