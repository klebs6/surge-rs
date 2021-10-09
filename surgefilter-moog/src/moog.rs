ix!();

pub struct LpMoogFilter<'sr> {
    pub tuner:  TunerHandle<'sr>, 
    pub srunit: SampleRateHandle<'sr>,
}


