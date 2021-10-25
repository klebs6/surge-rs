ix!();

use crate::{
    SurgeTuner,
    Scale,
    SurgeTuning,
    KeyboardMapping,
    traits::Retune,
    traits::KeyboardRemapper,
    traits::Note2Pitch,
};

///As in the rest of Surge432, TunerHandle is a facade to the outside world
///we can pass around a TunerHandle instead of exposing implementation details 
///of the SurgeTuner TunerHandle can be freely cloned and shared. Perhaps 
///we could use an Arc<Mutex<SurgeTuner>> to allow multithreaded tuning control updates
///
#[derive(Debug,Clone)]
pub struct TunerHandle<'sr> {
    inner: Rc<RefCell<SurgeTuner<'sr>>>,
}

impl TunerHandle<'sr> {
    pub fn new(srunit: &'sr SampleRateHandle<'sr>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(SurgeTuner::new(srunit))),
        }
    }
    #[inline] pub fn current_scale(&self) -> Scale {
        self.inner.borrow().current_scale.0.clone()
    }
    #[inline] pub fn current_tuning(&self) -> SurgeTuning {
        self.inner.borrow().current_tuning.0.clone()
    }
    #[inline] pub fn current_scale_count<T>(&self) -> T 
    where 
        T: TryFrom<i8>,
        <T as std::convert::TryFrom<i8>>::Error: std::fmt::Debug 
    {
        let val = self.inner.borrow().current_scale.count;
        assert!(val & 127 == val);
        T::try_from(val as i8).unwrap()
    }
    #[inline] pub fn current_tuning_is_standard(&self) -> bool {
        self.inner.borrow().current_tuning.is_standard_tuning
    }
    #[inline] pub fn current_mapping_is_standard(&self) -> bool {
        self.inner.borrow().current_mapping.is_standard_mapping
    }
    #[inline] pub fn current_scale_raw_contents(&self) -> TuningData {
        self.inner.borrow().current_scale.raw_text.clone()
    }
    #[inline] pub fn current_mapping_raw_contents(&self) -> MappingData {
        self.inner.borrow().current_mapping.raw_text.clone()
    }
    #[inline] pub fn retune_to_scale(&mut self, scale: &Scale) -> bool {
        self.inner.borrow_mut().retune_to_scale(scale)
    }
    #[inline] pub fn retune_to_standard_tuning(&mut self) -> bool {
        self.inner.borrow_mut().retune_to_standard_tuning()
    }
    #[inline] pub fn remap_to_keyboard(&mut self, kb: &KeyboardMapping) -> bool {
        self.inner.borrow_mut().remap_to_keyboard(kb)
    }
    #[inline] pub fn remap_to_standard_keyboard(&mut self) -> bool {
        self.inner.borrow_mut().remap_to_standard_keyboard()
    }
    #[inline] pub fn n2p<T:MyFloat, const IGNORE_TUNING: bool>(&self, x: T) -> T {
        self.inner.borrow().n2p::<T,IGNORE_TUNING>(x)
    }
    #[inline] pub fn n2pinv<T: MyFloat, const IGNORE_TUNING: bool>(&self, x: T) -> T {
        self.inner.borrow().n2pinv::<T,IGNORE_TUNING>(x)
    }
    #[inline] pub fn note_to_omega<T: MyFloat, const IGNORE_TUNING: bool>(&self, x: T) -> (T, T) {
        self.inner.borrow().note_to_omega::<T,IGNORE_TUNING>(x)
    }
    #[inline] pub fn n2p_tuningctr<T: MyFloat>(&self, x: T) -> T {
        self.inner.borrow().n2p_tuningctr::<T>(x)
    }
    #[inline] pub fn n2pinv_tuningctr<T: MyFloat>(&self, x: T) -> T {
        self.inner.borrow().n2pinv_tuningctr::<T>(x)
    }
    #[inline] pub fn get_tablepitch<IDX>(&self, idx: IDX) -> f64 
    where 
        IDX: TryInto<usize>,
        <IDX as std::convert::TryInto<usize>>::Error: std::fmt::Debug
    {
        let idx: usize = idx.try_into().unwrap(); 
        self.inner.borrow().tables.table_pitch[idx]
    }
    #[inline] pub fn pitch2omega<T: MyFloat>(&self, x: T) -> T {
        self.inner.borrow().pitch2omega::<T>(x)
    }
}
