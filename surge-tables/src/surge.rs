ix!();

use crate::{
    *,
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

impl EnvelopeRateLpf for SurgeTables<'sr> {

    #[inline] fn envelope_rate_lpf(&self, x: f32) -> f32 {
        self.envelope.envelope_rate_lpf(x) 
    }
}

impl EnvelopeRateLinear for SurgeTables<'sr> {

    #[inline] fn envelope_rate_linear(&self, x: f32) -> f32 {
        self.envelope.envelope_rate_linear(x) 
    }
}

impl DbToLinear for SurgeTables<'sr> {

    #[inline] fn db_to_linear(&self, x: f32) -> f32 { 
        self.gain.db_to_linear(x) 
    }
}

impl ClipScale for SurgeTables<'sr> {

    #[inline] fn clipscale(&self, freq: f32, subtype: FilterSubType) -> f32 { 
        self.gain.clipscale(freq,subtype) 
    }
}

impl LookupWaveshape for SurgeTables<'sr> {

    #[inline] fn lookup_waveshape(&self, entry: i32, x: f32) -> f32 {
        self.waveshape.lookup_waveshape(entry,x) 
    }
}

impl LookupWaveshapeWarp for SurgeTables<'sr> {

    #[inline] fn lookup_waveshape_warp(&self, entry: i32, x: f32) -> f32 {
        self.waveshape.lookup_waveshape_warp(entry,x) 
    }
}

impl SincTable1X for SurgeTables<'sr> {

    #[inline] fn _sinctable_1x(&self, idx: usize) -> f32  
    {
        self.sinc.table_1x[idx]
    }
}

impl SincTable for SurgeTables<'sr> {

    #[inline] fn _sinctable(&self, idx: usize) -> f32  
    {
        self.sinc.table[idx]
    }
}

impl SincTableI16 for SurgeTables<'sr> {

    #[inline] fn _sinctable_i16(&self, idx: usize) -> i16  
    {
        self.sinc.table_i16[idx]
    }
}

impl SincTable1XPtr for SurgeTables<'sr> {

    #[inline] fn _sinctable_1x_ptr(&self, idx: usize) -> *const f32  
    {
        &self.sinc.table_1x[idx]
    }
}

impl SincTablePtr for SurgeTables<'sr> {

    #[inline] fn _sinctable_ptr(&self, idx: usize) -> *const f32  
    {
        &self.sinc.table[idx]
    }
}

impl SincTableI16Ptr for SurgeTables<'sr> {

    #[inline] fn _sinctable_i16_ptr(&self, idx: usize) -> *const i16  
    {
        &self.sinc.table_i16[idx]
    }
}

impl GetWaveshaperPtr for SurgeTables<'sr> {

    #[inline] fn _get_waveshaper_ptr(&self, idx: usize, offset: isize) -> *const f32 
    {
        unsafe {
            self.waveshape.table[idx].as_ptr().offset(offset)
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
