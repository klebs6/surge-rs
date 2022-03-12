ix!();

use crate::*;

#[enum_dispatch(
    GetVuFalloff,
    Ms2Samples,
    GetSampleRate,
    SetSampleRate,
)]
#[derive(Debug)]
pub enum MaybeOwningSampleRateUnit {
    Owning(SampleRateUnit),
    NonOwning(SampleRateHandle),
}

#[derive(Debug,Clone)]
pub struct SampleRateHandle {
    inner: Rc<RefCell<SampleRateUnit>>,
}

impl SampleRateHandle {

    pub fn new_with_samplerate(sr: f64) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SampleRateUnit::new_with_samplerate(sr))),
        }
    }
}

impl Default for SampleRateHandle {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SampleRateUnit::new_with_samplerate(48_000.0_f64))),
        }
    }
}

impl GetVuFalloff for SampleRateHandle {
    #[inline] fn vu_falloff(&self) -> f32 {
        self.inner.borrow().vu_falloff
    }
}

impl Ms2Samples for SampleRateHandle {

    /**
      from allpass filter
      */
    #[inline] fn ms_2_samples(&self, ms: f32, scale: f32) -> usize
    {
        let inner = self.inner.borrow();

        let sr = inner.samplerate.load( atomic::Ordering::SeqCst );

        let a: f32 =  sr * ms * 0.001;

        let b: f32 = a * scale;

        b as usize
    }
}

impl GetSampleRate for SampleRateHandle {

    #[inline] fn dsamplerate_os(&self)      -> f64 { self.inner.borrow().dsamplerate_os.load( atomic::Ordering::SeqCst ) }
    #[inline] fn dsamplerate(&self)         -> f64 { self.inner.borrow().dsamplerate.load( atomic::Ordering::SeqCst ) }
    #[inline] fn samplerate_os(&self)       -> f32 { self.inner.borrow().samplerate_os.load( atomic::Ordering::SeqCst ) }
    #[inline] fn samplerate(&self)          -> f32 { self.inner.borrow().samplerate.load( atomic::Ordering::SeqCst ) }
    #[inline] fn dsamplerate_os_inv(&self)  -> f64 { self.inner.borrow().dsamplerate_os_inv.load( atomic::Ordering::SeqCst ) }
    #[inline] fn dsamplerate_inv(&self)     -> f64 { self.inner.borrow().dsamplerate_inv.load( atomic::Ordering::SeqCst ) }
    #[inline] fn samplerate_os_inv(&self)   -> f32 { self.inner.borrow().samplerate_os_inv.load( atomic::Ordering::SeqCst ) }
    #[inline] fn samplerate_inv(&self)      -> f32 { self.inner.borrow().samplerate_inv.load( atomic::Ordering::SeqCst ) }

    /*
    #[inline] pub fn get_nyquist_pitch(&self)   -> f32 { 
        // include some margin for error 
        // (and to avoid denormals in IIR filter clamping)
        12.0 * (
            (0.75 * PI) 
            / (self.dsamplerate_os_inv() * 2 * PI * CONCERT_A_HZ)
        ).log2()
    }
    */
}

impl SetSampleRate for SampleRateHandle {

    #[inline] fn set_samplerate(&self, sr: f64) 
    {
        let sr32: f32 = sr as f32;

        let sros64 = sr * (OSC_OVERSAMPLING as f64);

        let inner = self.inner.borrow_mut();

        inner.samplerate.store(         sr32,         atomic::Ordering::SeqCst);
        inner.samplerate_inv.store(     1.0 / sr32,   atomic::Ordering::SeqCst);
        inner.dsamplerate.store(        sr,           atomic::Ordering::SeqCst);
        inner.dsamplerate_inv.store(    1.0 / sr,     atomic::Ordering::SeqCst);
        inner.dsamplerate_os.store(     sros64,       atomic::Ordering::SeqCst);
        inner.dsamplerate_os_inv.store( 1.0 / sros64, atomic::Ordering::SeqCst);
    }
}
