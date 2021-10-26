ix!();

use crate::{
    Emphasize,
    EmphasizeParam,
};

impl Emphasize {

    pub fn new(
        tuner:  & TunerHandle,
        tables: & TablesHandle,
        srunit: & SampleRateHandle) -> Self 
    {
        Self {
            pre:              Align16(HalfRateFilterSSE::default()),
            post:             Align16(HalfRateFilterSSE::default()),
            ty:               Align16(LipolPs::default()),
            outgain:          Align16(LipolPs::default()),
            ringout:          Ringout::blocks(50),
            params:           EmphasizeParam::new_runtime(),
            eq:               BiquadFilter::new(tuner,tables,srunit),
            block_increment:  0,
            left:             0.0,
            right:            0.0,
        }
    }
}

impl Init for Emphasize {
    fn init(&mut self) {
        todo!();
    }
}
