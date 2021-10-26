ix!();

use crate::{
    Eq3BandParam,
    Eq3BandParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Eq3Band {

    pub gain:             Align16<LipolPs>,
    pub band1:            BiquadFilter,
    pub band2:            BiquadFilter,
    pub band3:            BiquadFilter,

    /// block increment (to keep track of 
    /// events not occurring every n blocks)
    pub block_increment:  i32, 
    pub tables:           MaybeOwningTablesHandle,

    pub ringout:          Ringout,
    pub params:           Eq3BandParamArrayRT,
}

no_op!  [Eq3Band, ProcessOnlyControl];
effect! [Eq3Band,       Eq3BandParam];
no_op!  [Eq3Band,            Suspend];
name!   [Eq3Band,               "eq"]; 
