ix!();

use crate::MidiChannelState;

#[derive(Debug)]
pub struct HoldPedalUnit {
    pub hold_buffer:  Vec<(i32, i32)>,
    pub sine:         QuadrOsc,
    pub demo_counter: i32,
    pub phantom:      PhantomData<& u8>,
}

impl Default for HoldPedalUnit {

    fn default() -> Self {
        Self {
            hold_buffer:  Vec::new(), 
            sine:         QuadrOsc::new(),
            demo_counter: 10,
            phantom:       Default::default(),
        }
    }
}

impl HoldPedalUnit {

    pub fn reset(&mut self, channel: u8, key: u8) {

        for pair in self.hold_buffer.iter_mut() {
            if (pair.0, pair.1) == (channel.into(), key.into()) {
                pair.0 = -1;
                pair.1 = -1;

            }
        }
    }

    /// # Safety
    ///
    /// need to ensure channel_state is valid across the 
    /// queried range of channels (held in the hold_buffer)
    #[inline] pub unsafe fn purge(&mut self, channel_state: *const MidiChannelState) -> Vec<(i32,i32)> {

        let mut retain_buffer: Vec::<(i32,i32)> = Vec::new();
        let mut release_buffer: Vec::<(i32,i32)> = Vec::new();

        for (channel, key) in self.hold_buffer.iter() {

            if *channel < 0 || *key < 0 {
                panic!("caught tricky double release condition");
            } 

            if !(*channel_state.add(0)).hold && !(*channel_state.add(*channel as usize)).hold {
                release_buffer.push((*channel,*key));

            } else {
                retain_buffer.push((*channel,*key));
            }
        }
        self.hold_buffer = retain_buffer;
        release_buffer
    }
}

#[derive(Debug,Clone)]
pub struct HoldPedalUnitHandle {
    inner: Rc<RefCell<HoldPedalUnit>>,
}

impl Default for HoldPedalUnitHandle {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(HoldPedalUnit::default())),
        }
    }
}

impl HoldPedalUnitHandle {

    /// # Safety
    ///
    /// need to make sure we can access every queried channel_state
    /// through the poitner
    #[inline] pub unsafe fn purge(&mut self, channel_state: *const MidiChannelState) -> Vec<(i32,i32)> {
        self.inner.borrow_mut().purge(channel_state)
    }

    #[inline] pub fn set_quadrosc_rate(&mut self, rate: f64) {
        self.inner.borrow_mut().sine.set_rate(rate);
    }

    #[inline] pub fn clear_holdbuffer(&mut self) {
        self.inner.borrow_mut().hold_buffer.clear();
    }

    #[inline] pub fn reset(&mut self, channel: u8, key: u8) {
        self.inner.borrow_mut().reset(channel,key);
    }

    #[inline] pub fn push(&mut self, channel: u8, key: u8) {
        self.inner.borrow_mut().hold_buffer.push((channel as i32, key as i32));
    }
}
