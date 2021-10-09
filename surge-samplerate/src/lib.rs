#![feature(in_band_lifetimes)]
use std::marker::PhantomData;
use atomic_float::{AtomicF64,AtomicF32};
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic;
use surge_constants::OSC_OVERSAMPLING;

#[derive(Debug)]
struct SampleRateUnit<'sr> {
    samplerate:         AtomicF32,
    samplerate_inv:     AtomicF32,
    samplerate_os:      AtomicF32,
    samplerate_os_inv:  AtomicF32,

    dsamplerate:        AtomicF64,
    dsamplerate_inv:    AtomicF64,
    dsamplerate_os:     AtomicF64,
    dsamplerate_os_inv: AtomicF64,
    vu_falloff:         f32,
    phantom:            PhantomData<&'sr i32>,
}

impl SampleRateUnit<'sr> {
    fn new(sr: f64) -> Self {

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

#[derive(Debug,Clone)]
pub struct SampleRateHandle<'sr> {
    inner: Rc<RefCell<SampleRateUnit<'sr>>>,
}

impl Default for SampleRateHandle<'sr> {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SampleRateUnit::new(48_000.0_f64))),
        }
    }
}

impl SampleRateHandle<'sr> {

    pub fn new_with_samplerate(sr: f64) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SampleRateUnit::new(sr))),
        }
    }

    ///from allpass filter
    #[inline] pub fn ms_2_samples(&self, ms: f32, scale: f32) -> usize
    {
        let sr = self.inner.borrow().samplerate.load( atomic::Ordering::SeqCst );

        let a: f32 =  sr * ms * 0.001;

        let b: f32 = a * scale;

        b as usize
    }

    #[inline] pub fn dsamplerate_os(&self)      -> f64 { self.inner.borrow().dsamplerate_os.load( atomic::Ordering::SeqCst ) }
    #[inline] pub fn dsamplerate(&self)         -> f64 { self.inner.borrow().dsamplerate.load( atomic::Ordering::SeqCst ) }
    #[inline] pub fn samplerate_os(&self)       -> f32 { self.inner.borrow().samplerate_os.load( atomic::Ordering::SeqCst ) }
    #[inline] pub fn samplerate(&self)          -> f32 { self.inner.borrow().samplerate.load( atomic::Ordering::SeqCst ) }
    #[inline] pub fn dsamplerate_os_inv(&self)  -> f64 { self.inner.borrow().dsamplerate_os_inv.load( atomic::Ordering::SeqCst ) }
    #[inline] pub fn dsamplerate_inv(&self)     -> f64 { self.inner.borrow().dsamplerate_inv.load( atomic::Ordering::SeqCst ) }
    #[inline] pub fn samplerate_os_inv(&self)   -> f32 { self.inner.borrow().samplerate_os_inv.load( atomic::Ordering::SeqCst ) }
    #[inline] pub fn samplerate_inv(&self)      -> f32 { self.inner.borrow().samplerate_inv.load( atomic::Ordering::SeqCst ) }
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

    #[inline] pub fn vu_falloff(&self) -> f32 {
        self.inner.borrow().vu_falloff
    }

    #[inline] pub fn set_samplerate(&self, sr: f64) 
    {
        let sr32: f32 = sr as f32;

        let sros64 = sr * (OSC_OVERSAMPLING as f64);

        self.inner.borrow_mut().samplerate.store(         sr32,         atomic::Ordering::SeqCst);
        self.inner.borrow_mut().samplerate_inv.store(     1.0 / sr32,   atomic::Ordering::SeqCst);
        self.inner.borrow_mut().dsamplerate.store(        sr,           atomic::Ordering::SeqCst);
        self.inner.borrow_mut().dsamplerate_inv.store(    1.0 / sr,     atomic::Ordering::SeqCst);
        self.inner.borrow_mut().dsamplerate_os.store(     sros64,       atomic::Ordering::SeqCst);
        self.inner.borrow_mut().dsamplerate_os_inv.store( 1.0 / sros64, atomic::Ordering::SeqCst);
    }
}
