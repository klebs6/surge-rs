ix!();

use crate::{
    Distortion,
    DISTORTION_OS,
    DistortionParam,
};

impl Init for Distortion<'sr> {

    fn init(&mut self) {

        self.update_all_bands();
        self.zero_drives();

        self.band1.suspend();
        self.band2.suspend();
        self.lp1.suspend();
        self.lp2.suspend();

        self.bi = 0;
        self.left  = 0.0;
        self.right = 0.0;
    }
}

impl Distortion<'sr> {

    pub fn new(
        tuner:  &'sr TunerHandle<'sr>,
        tables: &'sr TablesHandle<'sr>,
        srunit: &'sr SampleRateHandle<'sr>) -> Self 
    {
        Self {
            hr_a:     Align16(HalfRateFilterSSE::new(3,false)),
            hr_b:     Align16(HalfRateFilterSSE::new(3,true)),
            drive:    Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            outgain:  Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            ringout:  Ringout::blocks(1000),
            params:   DistortionParam::new_runtime(), 
            band1:    BiquadFilter::new(tuner,tables,srunit),
            band2:    BiquadFilter::new(tuner,tables,srunit),
            lp1:      BiquadFilter::new_with_blocksize(tuner, tables, srunit, BLOCK_SIZE * DISTORTION_OS),
            lp2:      BiquadFilter::new_with_blocksize(tuner, tables, srunit, BLOCK_SIZE * DISTORTION_OS),
            bi:       0,
            left:     0.0,
            right:    0.0,
            tables:   tables.clone(),
        }
    }
}
