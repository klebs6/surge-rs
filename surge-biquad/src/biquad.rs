crate::ix!();

#[derive(Debug,Clone)]
pub struct BiquadFilter {
    pub a1:         Align16<VLag>,
    pub a2:         Align16<VLag>,
    pub b0:         Align16<VLag>,
    pub b1:         Align16<VLag>,
    pub b2:         Align16<VLag>,
    pub reg0:       Align16<VDouble>,
    pub reg1:       Align16<VDouble>,
    pub first_run:  bool,
    pub tuner:      TunerHandle,
    pub tables:     TablesHandle,
    pub srunit:     SampleRateHandle,
}

impl Suspend for BiquadFilter {}

impl BiquadFilter {

    pub fn coeff_process(&mut self) {
        self.a1.process();
        self.a2.process();
        self.b0.process();
        self.b1.process();
        self.b2.process();
    }

    pub fn coeff_instantize(&mut self)
    {
        self.a1.instantize();
        self.a2.instantize();
        self.b0.instantize();
        self.b1.instantize();
        self.b2.instantize();
    }
}

impl SetBlockSize for BiquadFilter {

    fn set_blocksize(&mut self, _bs: i32) {
        /*
        //this was commented in the C but now we are
        //using vlag. why? what is the difference?
        self.a1.set_block_size(bs);
        self.a2.set_block_size(bs);
        self.b0.set_block_size(bs);
        self.b1.set_block_size(bs);
        self.b2.set_block_size(bs);
        */
    }
}

impl Initialize for BiquadFilter {

    fn init(&mut self) -> Result<(),SurgeError> {
        self.a1        = Align16(VLag::new_x87());
        self.a2        = Align16(VLag::new_x87());
        self.b0        = Align16(VLag::new_x87());
        self.b1        = Align16(VLag::new_x87());
        self.b2        = Align16(VLag::new_x87());
        self.reg0      = Align16(VDouble::default());
        self.reg1      = Align16(VDouble::default());
        self.first_run = true;

        Ok(())
    }
}

impl BiquadFilter {

    pub fn new(
        tuner:  & TunerHandle,
        tables: & TablesHandle,
        srunit: & SampleRateHandle,
    ) -> Self {

        Self {
            tuner:      tuner.clone(),
            tables:     tables.clone(),
            srunit:     srunit.clone(),
            a1:         Align16(VLag::new_x87()),
            a2:         Align16(VLag::new_x87()),
            b0:         Align16(VLag::new_x87()),
            b1:         Align16(VLag::new_x87()),
            b2:         Align16(VLag::new_x87()),
            reg0:       Align16(VDouble::default()),
            reg1:       Align16(VDouble::default()),
            first_run:  true
        }
    }

    pub fn new_with_blocksize<T: TryInto<i32>>(
        tuner:  & TunerHandle,
        tables: & TablesHandle,
        srunit: & SampleRateHandle,
        bs: T) -> Self 
        where <T as std::convert::TryInto<i32>>::Error: std::fmt::Debug
    {
        let mut x = Self::new(tuner,tables,srunit);
        x.set_blocksize(bs.try_into().unwrap());
        x
    }
}
