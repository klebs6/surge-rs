crate::ix!();

#[enum_dispatch(
    Initialize,
    Note2Pitch,
    CurrentScaleCount,
    CurrentScale,
    CurrentTuning,
    CurrentTuningIsStandard,
    CurrentMappingIsStandard,
    CurrentScaleRawContents,
    CurrentMappingRawContents,
    RemapKeyboard,
    RetuneToScale,
    RetuneToStandardTuning,
    RemapToStandardKeyboard,
    GetTablePitch,
)]
#[derive(Debug,Clone)]
pub enum MaybeOwningTunerHandle {
    Owning(SurgeTuner),
    NonOwning(TunerHandle),
}

/**
  |As in the rest of Surge, TunerHandle is
  |a facade to the outside world we can pass
  |around a TunerHandle instead of exposing
  |implementation details of the SurgeTuner
  |TunerHandle can be freely cloned and
  |shared. Perhaps we could use an
  |Arc<Mutex<SurgeTuner>> to allow multithreaded
  |tuning control updates
  |
  */
#[derive(Debug,Clone)]
pub struct TunerHandle {
    inner: Rc<RefCell<SurgeTuner>>,
}

impl TunerHandle {
    pub fn new(srunit: &SampleRateHandle) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SurgeTuner::new(srunit))),
        }
    }
}

impl Initialize for TunerHandle {

    fn init(&mut self) {
        self.inner.borrow_mut().init()
    }
}

impl Note2Pitch for TunerHandle {

    #[inline] fn n2p<T:MyFloat, const IGNORE_TUNING: bool>(&self, x: T) -> T {
        self.inner.borrow().n2p::<T,IGNORE_TUNING>(x)
    }

    #[inline] fn n2pinv<T: MyFloat, const IGNORE_TUNING: bool>(&self, x: T) -> T {
        self.inner.borrow().n2pinv::<T,IGNORE_TUNING>(x)
    }

    #[inline] fn note_to_omega<T: MyFloat, const IGNORE_TUNING: bool>(&self, x: T) -> (T, T) {
        self.inner.borrow().note_to_omega::<T,IGNORE_TUNING>(x)
    }

    #[inline] fn n2p_tuningctr<T: MyFloat>(&self, x: T) -> T {
        self.inner.borrow().n2p_tuningctr::<T>(x)
    }

    #[inline] fn n2pinv_tuningctr<T: MyFloat>(&self, x: T) -> T {
        self.inner.borrow().n2pinv_tuningctr::<T>(x)
    }

    #[inline] fn pitch2omega<T: MyFloat>(&self, x: T) -> T {
        self.inner.borrow().pitch2omega::<T>(x)
    }
}

impl CurrentScaleCount for TunerHandle {

    #[inline] fn current_scale_count<T>(&self) -> T 
    where 
        T: TryFrom<i8>,
        <T as std::convert::TryFrom<i8>>::Error: std::fmt::Debug 
    {
        let val = self.inner.borrow().current_scale.count;
        assert!(val & 127 == val);
        T::try_from(val as i8).unwrap()
    }
}

impl CurrentScale for TunerHandle {

    #[inline] fn current_scale(&self) -> Scale {
        self.inner.borrow().current_scale.0.clone()
    }
}

impl CurrentTuning for TunerHandle {

    #[inline] fn current_tuning(&self) -> SurgeTuning {
        self.inner.borrow().current_tuning.0.clone()
    }
}

impl CurrentTuningIsStandard for TunerHandle {

    #[inline] fn current_tuning_is_standard(&self) -> bool {
        self.inner.borrow().current_tuning.is_standard_tuning
    }
}

impl CurrentMappingIsStandard for TunerHandle {

    #[inline] fn current_mapping_is_standard(&self) -> bool {
        self.inner.borrow().current_mapping.is_standard_mapping
    }
}

impl CurrentScaleRawContents for TunerHandle {

    #[inline] fn current_scale_raw_contents(&self) -> TuningData {
        self.inner.borrow().current_scale.raw_text.clone()
    }
}

impl CurrentMappingRawContents for TunerHandle {

    #[inline] fn current_mapping_raw_contents(&self) -> MappingData {
        self.inner.borrow().current_mapping.raw_text.clone()
    }
}

impl RetuneToScale for TunerHandle {

    #[inline] fn retune_to_scale(&mut self, scale: &Scale) -> bool {
        self.inner.borrow_mut().retune_to_scale(scale)
    }
}

impl RetuneToStandardTuning for TunerHandle { 

    #[inline] fn retune_to_standard_tuning(&mut self) {
        self.inner.borrow_mut().retune_to_standard_tuning()
    }
}

impl RemapKeyboard for TunerHandle {

    #[inline] fn remap_to_keyboard(&mut self, kb: &KeyboardMapping) -> bool {
        self.inner.borrow_mut().remap_to_keyboard(kb)
    }
}

impl RemapToStandardKeyboard for TunerHandle {

    #[inline] fn remap_to_standard_keyboard(&mut self) -> bool {
        self.inner.borrow_mut().remap_to_standard_keyboard()
    }
}

impl GetTablePitch for TunerHandle {

    #[inline] fn get_tablepitch<IDX>(&self, idx: IDX) -> f64 
    where 
        IDX: TryInto<usize>,
        <IDX as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        self.inner.borrow().tables.table_pitch[idx]
    }
}
