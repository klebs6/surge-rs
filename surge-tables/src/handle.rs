crate::ix!();

/**
  | We use this struct because any entity which
  | may share a handle to SurgeTables may also own
  | it as a standalone unit
  */
#[enum_dispatch(
    DbToLinear,
    Init,
    ClipScale,
    LookupWaveshape,
    LookupWaveshapeWarp,
    EnvelopeRateLpf,
    EnvelopeRateLinear,
    SincTable1X,
    SincTable,
    SincTableI16,
    SincTable1XPtr,
    SincTablePtr,
    SincTableI16Ptr,
    GetWaveshaperPtr,
)]
#[derive(Debug,Clone)]
pub enum MaybeOwningTablesHandle {
    Owning(SurgeTables),
    NonOwning(TablesHandle),
}

#[derive(Debug,Clone)]
pub struct TablesHandle {
    inner: Rc<RefCell<SurgeTables>>,
}

impl TablesHandle {

    pub fn new(srunit: &SampleRateHandle) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SurgeTables::new(srunit))),
        }
    }
}

impl DbToLinear for TablesHandle {

    #[inline] fn db_to_linear(&self, x: f32) -> f32 { 
        let inner = self.inner.borrow();
        inner.db_to_linear(x) 
    }
}

impl Init for TablesHandle {

    #[inline] fn init(&mut self) { 
        self.inner.borrow_mut().init();
    }
}

impl ClipScale for TablesHandle {

    #[inline] fn clipscale(&self, freq: f32, subtype: FilterSubType) -> f32 { 

        let inner = self.inner.borrow();

        inner.clipscale(freq,subtype) 
    }
}

impl LookupWaveshape for TablesHandle {

    #[inline] fn lookup_waveshape(&self, entry: i32, x: f32) -> f32 {
        self.inner.borrow().lookup_waveshape(entry,x) 
    }
}

impl LookupWaveshapeWarp for TablesHandle {

    #[inline] fn lookup_waveshape_warp(&self, entry: i32, x: f32) -> f32 {
        self.inner.borrow().lookup_waveshape_warp(entry,x) 
    }
}

impl EnvelopeRateLpf for TablesHandle {

    #[inline] fn envelope_rate_lpf(&self, x: f32) -> f32 {
        self.inner.borrow().envelope_rate_lpf(x) 
    }
}

impl EnvelopeRateLinear for TablesHandle {

    #[inline] fn envelope_rate_linear(&self, x: f32) -> f32 {
        self.inner.borrow().envelope_rate_linear(x) 
    }
}

impl SincTable1X for TablesHandle {

    #[inline] fn _sinctable_1x(&self, idx: usize) -> f32  
    {
        self.inner.borrow().sinctable_1x(idx)
    }
}

impl SincTable for TablesHandle {

    #[inline] fn _sinctable(&self, idx: usize) -> f32  
    {
        self.inner.borrow().sinctable(idx)
    }
}

impl SincTableI16 for TablesHandle {

    #[inline] fn _sinctable_i16(&self, idx: usize) -> i16  
    {
        self.inner.borrow().sinctable_i16(idx)
    }
}

impl SincTable1XPtr for TablesHandle {

    #[inline] fn _sinctable_1x_ptr(&self, idx: usize) -> *const f32  
    {
        self.inner.borrow().sinctable_1x_ptr(idx)
    }
}

impl SincTablePtr for TablesHandle {

    #[inline] fn _sinctable_ptr(&self, idx: usize) -> *const f32  
    {
        self.inner.borrow().sinctable_ptr(idx)
    }
}

impl SincTableI16Ptr for TablesHandle {

    #[inline] fn _sinctable_i16_ptr(&self, idx: usize) -> *const i16  
    {
        self.inner.borrow().sinctable_i16_ptr(idx)
    }
}

impl GetWaveshaperPtr for TablesHandle {

    #[inline] fn _get_waveshaper_ptr(&self, idx: usize, offset: isize) -> *const f32 
    {
        self.inner.borrow().get_waveshaper_ptr(idx,offset)
    }
}
