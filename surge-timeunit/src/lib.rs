#![feature(in_band_lifetimes)]

use surge_samplerate::SampleRateHandle;
use surge_traits::Init;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug,Clone)]
pub struct TimeUnit<'sr> {
    pub ppq_pos:              f64,
    pub tempo:                f64,
    pub time_sig_numerator:   i32,
    pub time_sig_denominator: i32,
    pub temposyncratio:       f32,
    pub temposyncratio_inv:   f32, //1.0 is 120 BPM
    pub songpos:              f64,
    pub srunit:               SampleRateHandle<'sr>,
}

impl Init for TimeUnit<'sr> {
    fn init(&mut self) {
        self.temposyncratio = 1.0;

        // Use this as a sentinel (since it was not initialized 
        // prior to 1.6.5 this was the value at least win and mac had). 
        // #1444
        self.temposyncratio_inv = 0.0;

        self.songpos = 0.0;
    }
}

impl TimeUnit<'sr> {
    pub fn new(srunit: &'sr SampleRateHandle<'sr>) -> Self {
        let mut x = Self {
            ppq_pos:                0.0,
            tempo:                  0.0,
            time_sig_numerator:       4,
            time_sig_denominator:     4,
            temposyncratio:         1.0,
            temposyncratio_inv:     1.0,
            songpos:                0.0,
            srunit:                 srunit.clone(),
        };
        x.init();
        x
    }

    pub fn update(&mut self) {
        self.songpos            = self.ppq_pos;
        self.temposyncratio     = (self.tempo / 120.0) as f32;
        self.temposyncratio_inv = 1.0 / self.temposyncratio;
    }
}

#[derive(Debug,Clone)]
pub struct TimeUnitHandle<'sr> {
    inner: Rc<RefCell<TimeUnit<'sr>>>,
}

impl TimeUnitHandle<'sr> {
    pub fn new(srunit: &'sr SampleRateHandle<'sr>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(TimeUnit::new(srunit))),
        }
    }

    #[inline] pub fn temposyncratio(&self)     -> f32 { self.inner.borrow().temposyncratio }
    #[inline] pub fn tempo(&self)              -> f64 { self.inner.borrow().tempo }
    #[inline] pub fn temposyncratio_inv(&self) -> f32 { self.inner.borrow().temposyncratio_inv }
    #[inline] pub fn songpos(&self)            -> f64 { self.inner.borrow().songpos }
    #[inline] pub fn ppq_pos(&self)            -> f64 { self.inner.borrow().ppq_pos }

    #[inline] pub fn update(&mut self) {
        self.inner.borrow_mut().update();
    }

    #[inline] pub fn set_temposyncratio(&mut self, val: f32) {
        self.inner.borrow_mut().temposyncratio = val;
    }

    #[inline] pub fn set_temposyncratio_inv(&mut self, val: f32) {
        self.inner.borrow_mut().temposyncratio_inv = val;
    }

    #[inline] pub fn set_songpos(&mut self, val: f64) {
        self.inner.borrow_mut().songpos = val;
    }
}
