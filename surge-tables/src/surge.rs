ix!();

use crate::{
    envelope::EnvelopeTables,
    gain::GainTables,
    sinc::SincTables,
    sine::SineTables,
    waveshape::WaveshapeTables,
};

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct SurgeTables<'sr> {
    pub envelope:    Align16<EnvelopeTables<'sr>>,
    pub gain:        Align16<GainTables>,
    pub sinc:        Align16<SincTables>,
    pub sine:        Align16<SineTables>,
    pub waveshape:   Align16<WaveshapeTables>,
}

impl SurgeTables<'sr> {
    pub fn new(srunit: &'sr SampleRateHandle<'sr>) -> Self {
        Self {
            envelope:   Align16(EnvelopeTables::new(srunit)),
            gain:       Align16(GainTables::default()),
            sinc:       Align16(SincTables::default()),
            sine:       Align16(SineTables::default()),
            waveshape:  Align16(WaveshapeTables::default()),
        }
    }
}

impl Init for SurgeTables<'sr> {

    fn init(&mut self) {
        self.envelope.init();
        self.gain.init();
        self.sinc.init();
        self.sine.init();
        self.waveshape.init();
    }
}
