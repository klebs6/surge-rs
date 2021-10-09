ix!();

use crate::{
    MIDIUnit,
    MidiChannelState,
    MidiKeyState,
};

#[derive(Debug,Clone)]
pub struct MIDIUnitHandle<'sr> {
    inner: Rc<RefCell<MIDIUnit<'sr>>>,
}

impl Default for MIDIUnitHandle<'sr> {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(MIDIUnit::default())),
        }
    }
}

impl MIDIUnitHandle<'sr> {

    #[inline] pub fn channel_state_ptr(&mut self, channel: u8) -> *mut MidiChannelState {
        assert!(channel < 16);
        unsafe {
            self.inner.borrow_mut().channel_state.as_mut_ptr().offset(channel as isize)
        }
    }
    #[inline] pub fn set_pitchbend(&self, channel: u8, val: i32) {
        self.inner.borrow_mut().channel_state[channel as usize].pitchbend = val;
    }
    #[inline] pub fn set_pitchbend_semitones(&self, channel: u8, val: f32) {
        self.inner.borrow_mut().channel_state[channel as usize].pitchbend_in_semitones = PitchBendValue(val);
    }
    #[inline] pub fn set_nrpn(&self, channel: u8, idx: usize, val: i32) {
        self.inner.borrow_mut().channel_state[channel as usize].nrpn[idx] = val;
    }
    #[inline] pub fn set_nrpn_v(&self, channel: u8, idx: usize, val: i32) {
        self.inner.borrow_mut().channel_state[channel as usize].nrpn_v[idx] = val;
    }
    #[inline] pub fn set_rpn(&self, channel: u8, idx: usize, val: i32) {
        self.inner.borrow_mut().channel_state[channel as usize].rpn[idx] = val;
    }
    #[inline] pub fn set_rpn_v(&self, channel: u8, idx: usize, val: i32) {
        self.inner.borrow_mut().channel_state[channel as usize].rpn_v[idx] = val;
    }
    #[inline] pub fn get_nrpn(&self, channel: u8, idx: usize) -> i32 {
        self.inner.borrow().channel_state[channel as usize].nrpn[idx]
    }
    #[inline] pub fn get_nrpn_v(&self, channel: u8, idx: usize) -> i32 {
        self.inner.borrow().channel_state[channel as usize].nrpn_v[idx]
    }
    #[inline] pub fn get_rpn(&self, channel: u8, idx: usize) -> i32 {
        self.inner.borrow().channel_state[channel as usize].rpn[idx]
    }
    #[inline] pub fn get_rpn_v(&self, channel: u8, idx: usize) -> i32 {
        self.inner.borrow().channel_state[channel as usize].rpn_v[idx]
    }
    #[inline] pub fn nrpn_last(&self, channel: u8) -> bool {
        self.inner.borrow().nrpn_last(channel)
    }
    #[inline] pub fn set_nrpn_last(&self, channel: u8, val: bool) {
        self.inner.borrow_mut().channel_state[channel as usize].nrpn_last = val;
    }
    #[inline] pub fn save_midi_controllers(&mut self) {
        self.inner.borrow_mut().save_midi_controllers()
    }
    #[inline] pub fn load_midi_controllers(&mut self) {
        self.inner.borrow_mut().load_midi_controllers()
    }
    #[inline] pub fn learn_param(&self) -> i32 {
        self.inner.borrow().learn_param
    }
    #[inline] pub fn set_learn_param(&self, val: i32) {
        self.inner.borrow_mut().learn_param = val;
    }
    #[inline] pub fn learn_custom(&self) -> i32 {
        self.inner.borrow().learn_custom
    }
    #[inline] pub fn set_learn_custom(&self, val: i32) {
        self.inner.borrow_mut().learn_custom = val;
    }
    #[inline] pub fn lastdetune(&self, channel: u8, key: u8) -> usize {
        self.inner.borrow().channel_state[channel as usize].key_state[key as usize].lastdetune
    }
    #[inline] pub fn keystate(&self, channel: u8, key: u8) -> i32 {
        self.inner.borrow().channel_state[channel as usize].key_state[key as usize].keystate
    }
    #[inline] pub fn keystate_ptr(&mut self, channel: u8, key: u8) -> *mut MidiKeyState {
        unsafe {
            self.inner.borrow_mut().channel_state[channel as usize].key_state.as_mut_ptr().add(key as usize)
        }
    }
    #[inline] pub fn hold(&self, channel: u8) -> bool {
        self.inner.borrow().channel_state[channel as usize].hold
    }
    #[inline] pub fn set_hold(&mut self, channel: u8, val: bool) {
        self.inner.borrow_mut().channel_state[channel as usize].hold = val;
    }
    #[inline] pub fn set_lastdetune(&mut self, channel: u8, key: u8, newvalue: u8) {
        self.inner.borrow_mut().channel_state[channel as usize].key_state[key as usize].lastdetune 
            = newvalue as usize;
    }
    #[inline] pub fn set_keystate(&mut self, channel: u8, key: u8, newvalue: u8) {
        self.inner.borrow_mut().channel_state[channel as usize].key_state[key as usize].keystate 
            = newvalue as i32;
    }
    #[inline] pub fn set_pressure(&mut self, 
        channel: u8, fval: f32) 
    {
        self.inner.borrow_mut().channel_state[channel as usize].pressure 
            = fval;
    }
    #[inline] pub fn set_pan(&mut self, 
        channel: u8, fval: f32) 
    {
        self.inner.borrow_mut().channel_state[channel as usize].pan 
            = fval;
    }
    #[inline] pub fn set_timbre(&mut self, 
        channel: u8, fval: f32) 
    {
        self.inner.borrow_mut().channel_state[channel as usize].timbre 
            = fval;
    }
    #[inline] pub fn clear_keystate(&mut self, channel: u8, key: u8) {
        self.inner.borrow_mut().channel_state[
            channel as usize].key_state[
                key as usize].keystate = 0;

    }
    #[inline] pub fn set_programs_changed(&mut self, val: bool) {
        self.inner.borrow_mut().midi_programs_have_changed = val;
    }
}
