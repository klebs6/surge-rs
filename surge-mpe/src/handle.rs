ix!();

use crate::MPEUnit;

#[derive(Debug,Clone)]
pub struct MPEUnitHandle {
    inner: Rc<RefCell<MPEUnit>>,
}

impl Default for MPEUnitHandle {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(MPEUnit::default())),
        }
    }
}

impl MPEUnitHandle {

    #[inline] pub fn set_num_voices(&mut self, val: u32) {
        self.inner.borrow_mut().num_voices = NumVoices(val);
    }

    #[inline] pub fn set_enabled(&mut self, val: bool) {
        self.inner.borrow_mut().enabled = MpeEnableSwitch(val);
    }

    #[inline] pub fn set_global_pitchbend_range(&mut self, val: f32) {
        self.inner.borrow_mut().global_pitchbend_range = PitchBendRange(val);
    }

    #[inline] pub fn set_pitchbend_range(&mut self, val: f32) {
        self.inner.borrow_mut().pitchbend_range = PitchBendRange(val);
    }

    #[inline] pub fn set_pitchbend(&mut self, val: f32) {
        self.inner.borrow_mut().pitchbend = PitchBendValue(val);
    }

    #[inline] pub fn pitchbend_range(&mut self) -> PitchBendRange {
        self.inner.borrow_mut().pitchbend_range
    }

    #[inline] pub fn pitchbend(&mut self) -> PitchBendValue {
        self.inner.borrow_mut().pitchbend
    }

    #[inline] pub fn global_pitchbend_range(&mut self) -> PitchBendRange {
        self.inner.borrow_mut().global_pitchbend_range
    }

    #[inline] pub fn enabled(&self) -> MpeEnableSwitch {
        self.inner.borrow().enabled
    }

    #[inline] pub fn get_mpe_main_channel(&self, channel: u8, key: u8) -> u8 {
        self.inner.borrow().get_mpe_main_channel(channel,key)
    }

    #[inline] pub fn set_lastkey<T>(&mut self, key: T) 
    where 
        T: TryInto<usize>, <T as TryInto<usize>>::Error: Debug,
    {
        let key:       usize = key.try_into().unwrap(); 

        assert!(key       & 127 == key);

        self.inner.borrow_mut().last_key = i32::try_from(key).unwrap();
    }

    #[inline] pub fn get_lastkey<T>(&mut self) -> T
    where 
        T: TryFrom<i32>, <T as TryFrom<i32>>::Error: Debug,
    {
        let key = self.inner.borrow().last_key;

        assert!(key & 127 == key);

        T::try_from(key).unwrap()
    }

    #[inline] pub fn get_poly_aftertouch<T>(&mut self, key: T) -> f32
    where 
        T: TryInto<usize>, <T as TryInto<usize>>::Error: Debug,
    {
        let key:       usize = key.try_into().unwrap(); 

        assert!(key       & 127 == key);

        self.inner.borrow_mut().poly_aftertouch[key]
    }

    #[inline] pub fn set_poly_aftertouch<T>(&mut self, key: T, val: f32)
    where 
        T: TryInto<usize>, <T as TryInto<usize>>::Error: Debug,
    {
        let key:       usize = key.try_into().unwrap(); 

        assert!(key       & 127 == key);

        self.inner.borrow_mut().poly_aftertouch[key] = val;
    }
}
