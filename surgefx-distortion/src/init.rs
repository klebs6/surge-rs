crate::ix!();

impl Initialize for Distortion {

    fn init(&mut self) -> Result<(),SurgeError> {

        self.update_all_bands();
        self.zero_drives();

        self.band1.suspend()?;
        self.band2.suspend()?;
        self.lp1.suspend()?;
        self.lp2.suspend()?;

        self.block_increment = 0;
        self.left  = 0.0;
        self.right = 0.0;

        Ok(())
    }
}

impl Distortion {

    pub fn new(
        tuner:  & TunerHandle,
        tables: & TablesHandle,
        srunit: & SampleRateHandle) -> Self 
    {
        Self {
            hr_a:            Align16(HalfRateFilterSSE::new(3,false)),
            hr_b:            Align16(HalfRateFilterSSE::new(3,true)),
            drive:           Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            outgain:         Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            ringout:         Ringout::blocks(1000),
            params:          DistortionParam::new_runtime(), 
            band1:           BiquadFilter::new(tuner,tables,srunit),
            band2:           BiquadFilter::new(tuner,tables,srunit),
            lp1:             BiquadFilter::new_with_blocksize(tuner, tables, srunit, BLOCK_SIZE * DISTORTION_OS),
            lp2:             BiquadFilter::new_with_blocksize(tuner, tables, srunit, BLOCK_SIZE * DISTORTION_OS),
            block_increment: 0,
            left:            0.0,
            right:           0.0,
            tables:          tables.clone(),
            wetblock:        WetBlock2::<128>::default(),
        }
    }
}
