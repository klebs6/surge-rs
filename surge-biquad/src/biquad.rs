ix!();

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
        //this was commented in the C
        //but now we are using vlag.
        //why? what is the difference?
        /*
           self.a1.set_block_size(bs);
           self.a2.set_block_size(bs);
           self.b0.set_block_size(bs);
           self.b1.set_block_size(bs);
           self.b2.set_block_size(bs);
           */
    }
}
