crate::ix!();

impl Initialize for BiquadFilter {

    fn init(&mut self) {
        self.a1        = Align16(VLag::new_x87());
        self.a2        = Align16(VLag::new_x87());
        self.b0        = Align16(VLag::new_x87());
        self.b1        = Align16(VLag::new_x87());
        self.b2        = Align16(VLag::new_x87());
        self.reg0      = Align16(VDouble::default());
        self.reg1      = Align16(VDouble::default());
        self.first_run = true;
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
