ix!();

use crate::{
    Conditioner,
    ConditionerParam,
};

impl Init for Conditioner<'sr> {

    fn init(&mut self) {

        self.update_bands();

        self.ef              = 0.0;
        self.bufpos          = 0;
        self.filtered_lamax  = 1.0;
        self.filtered_lamax2 = 1.0;
        self.gain            = 1.0;
        self.lamax           = Self::new_lamax();
        self.delayed         = Self::new_delayed();

        self.vu[0] = 0.0;
        self.vu[1] = 0.0;
        self.vu[2] = 1.0;
        self.vu[4] = 0.0;
        self.vu[5] = 0.0;
    }
}

impl Conditioner<'sr> {
    pub fn new(
        tuner:  &'sr TunerHandle<'sr>,
        tables: &'sr TablesHandle<'sr>,
        srunit: &'sr SampleRateHandle<'sr>)  -> Self {
        Self {
            ringout:          Ringout::blocks(100),
            params:           ConditionerParam::new_runtime(),
            amp_l:            Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            amp_r:            Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            width:            Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            postamp:          Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            band1:            BiquadFilter::new(tuner,tables,srunit),
            band2:            BiquadFilter::new(tuner,tables,srunit),
            ef:               0.0,
            a_rate:           LiPol::<f32>::default(),
            r_rate:           LiPol::<f32>::default(),
            lamax:            Self::new_lamax(),
            delayed:          Self::new_delayed(),
            bufpos:           0,
            filtered_lamax:   0.0,
            filtered_lamax2:  0.0,
            gain:             0.0,
            tables:           tables.clone(),
            srunit:           srunit.clone(),
            vu:               Self::new_vu(), 
        }
    }

    #[inline] pub fn new_lamax() -> A1d::<f32> {
        A1d::<f32>::zeros(CONDITIONER_LOOKAHEAD << 1)
    }

    #[inline] pub fn new_vu() -> A1d::<f32> {
        A1d::<f32>::zeros(CONDITIONER_NUM_VU_SLOTS)
    }

    #[inline] pub fn new_delayed() -> A2d::<f32> {
        A2d::<f32>::zeros((2,CONDITIONER_LOOKAHEAD))
    }
}
