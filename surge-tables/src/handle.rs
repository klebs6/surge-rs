ix!();

use crate::{
    SurgeTables,
    waveshape::WaveshapeTables,
};

#[derive(Debug,Clone)]
pub struct TablesHandle<'sr> {
    inner: Rc<RefCell<SurgeTables<'sr>>>,
}

impl TablesHandle<'sr> {

    pub fn new(srunit: &'sr SampleRateHandle<'sr>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SurgeTables::new(srunit))),
        }
    }

    #[inline] pub fn init(&mut self) { 
        self.inner.borrow_mut().init()
    }

    #[inline] pub fn db_to_linear(&self, x: f32) -> f32 { 
        self.inner.borrow().gain.db_to_linear(x) 
    }

    #[inline] pub fn clipscale(&self, freq: f32, subtype: FilterSubType) -> f32 { 
        self.inner.borrow().gain.clipscale(freq,subtype) 
    }

    #[inline] pub fn lookup_waveshape(&self, entry: i32, x: f32) -> f32 {
        self.inner.borrow().waveshape.lookup_waveshape(entry,x) 
    }

    #[inline] pub fn lookup_waveshape_warp(&self, entry: i32, x: f32) -> f32 {
        self.inner.borrow().waveshape.lookup_waveshape_warp(entry,x) 
    }

    #[inline] pub fn envelope_rate_lpf(&self, x: f32) -> f32 {
        self.inner.borrow().envelope.envelope_rate_lpf(x) 
    }

    #[inline] pub fn envelope_rate_linear(&self, x: f32) -> f32 {
        self.inner.borrow().envelope.envelope_rate_linear(x) 
    }

    #[inline] pub fn sinctable_1x<T: TryInto<usize>>(&self, idx: T) -> f32  
    where 
        <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        self.inner.borrow().sinc.table_1x[idx]
    }

    #[inline] pub fn sinctable<T: TryInto<usize>>(&self, idx: T) -> f32  
    where 
        <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        self.inner.borrow().sinc.table[idx]
    }

    #[inline] pub fn sinctable_i16<T: TryInto<usize>>(&self, idx: T) -> i16  
    where 
        <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        self.inner.borrow().sinc.table_i16[idx]
    }

    #[inline] pub fn sinctable_1x_ptr<T: TryInto<usize>>(&mut self, idx: T) -> *const f32  
    where 
        <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        &self.inner.borrow().sinc.table_1x[idx]
    }

    #[inline] pub fn sinctable_ptr<T: TryInto<usize>>(&mut self, idx: T) -> *const f32  
    where 
        <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        &self.inner.borrow().sinc.table[idx]
    }

    #[inline] pub fn sinctable_i16_ptr<T: TryInto<usize>>(&mut self, idx: T) -> *const i16  
    where 
        <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        &self.inner.borrow().sinc.table_i16[idx]
    }

    #[inline] pub fn get_waveshaper_ptr<T: TryInto<isize>>(&self, idx: usize, _offset: T) -> *const f32 
    where
        <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug
    {
        assert!(idx < WaveshapeTables::ntables());
        let offset: isize = idx.try_into().unwrap(); 
        assert!(offset & 0x3ff == offset);
        unsafe {
            self.inner.borrow().waveshape.table[idx].as_ptr().offset(offset)
        }
    }
}
