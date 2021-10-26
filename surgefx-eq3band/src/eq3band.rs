ix!();

use crate::{
    Eq3BandParam,
    Eq3BandParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Eq3Band<'sr> {

    pub gain:             Align16<LipolPs>,
    pub band1:            BiquadFilter<'sr>,
    pub band2:            BiquadFilter<'sr>,
    pub band3:            BiquadFilter<'sr>,

    /// block increment (to keep track of 
    /// events not occurring every n blocks)
    pub block_increment:  i32, 
    pub tables:           MaybeOwningTablesHandle<'sr>,

    pub ringout:          Ringout,
    pub params:           Eq3BandParamArrayRT,
}

no_op!  [Eq3Band<'sr>, ProcessOnlyControl];
effect! [Eq3Band<'sr>,       Eq3BandParam];
no_op!  [Eq3Band<'sr>,            Suspend];
name!   [Eq3Band<'sr>,               "eq"]; 
