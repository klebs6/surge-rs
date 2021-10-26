ix!();

#[derive(Debug)]
pub struct SampleRateUnit<'sr> {
    pub samplerate:         AtomicF32,
    pub samplerate_inv:     AtomicF32,
    pub samplerate_os:      AtomicF32,
    pub samplerate_os_inv:  AtomicF32,

    pub dsamplerate:        AtomicF64,
    pub dsamplerate_inv:    AtomicF64,
    pub dsamplerate_os:     AtomicF64,
    pub dsamplerate_os_inv: AtomicF64,
    pub vu_falloff:         f32,
    phantom:            PhantomData<&'sr i32>,
}

impl SampleRateUnit<'sr> {
    pub fn new_with_samplerate(sr: f64) -> Self {

        let sr32: f32 = sr as f32;

        let sros32 = sr32 * (OSC_OVERSAMPLING as f32);
        let sros64 = sr * (OSC_OVERSAMPLING as f64);

        Self {
            samplerate:            AtomicF32::new(sr32),
            samplerate_inv:        AtomicF32::new(1.0 / sr32),
            samplerate_os:         AtomicF32::new(sros32),
            samplerate_os_inv:     AtomicF32::new(1.0 / sros32),

            dsamplerate:           AtomicF64::new(sr),
            dsamplerate_inv:       AtomicF64::new(1.0 / sr),
            dsamplerate_os:        AtomicF64::new(sros64),
            dsamplerate_os_inv:    AtomicF64::new(1.0 / sros64),
            phantom:               Default::default(),

            //TODO should be samplerate-dependent 
            //(this is per 32-sample block at 44.1)
            vu_falloff:  0.997, 
        }
    }
}

