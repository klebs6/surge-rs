crate::ix!();

#[derive(Debug,Clone)]
#[name("emphasize")]
pub struct Emphasize {
    pub ringout:           Ringout,
    pub params:            EmphasizeParamArrayRT,
    pub pre:               Align16<HalfRateFilterSSE>,
    pub post:              Align16<HalfRateFilterSSE>,
    pub ty:                Align16<LipolPs>,
    pub outgain:           Align16<LipolPs>,
    pub eq:                BiquadFilter,

    ///(to keep track of events 
    ///not occurring every n blocks)
    pub block_increment:   i32,
    pub left:              f32,
    pub right:             f32,
}

impl Reset for Emphasize {

    fn reset(&mut self) {
        todo!();
    }
}

no_op!     [Emphasize, ProcessOnlyControl];
effect!    [Emphasize,     EmphasizeParam];
no_op!     [Emphasize,            Suspend];
no_update! [Emphasize                    ];
