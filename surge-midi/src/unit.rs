ix!();
use crate::{
    MidiChannelState,
};

#[derive(Debug)]
pub struct MIDIUnit<'sr> {
    pub midi_programs_have_changed: bool, 
    pub learn_param:    i32,
    pub learn_custom:   i32,
    pub channel_state:  [MidiChannelState; 16],
    pub phantom:        PhantomData<&'sr u8>,
}

impl Default for MIDIUnit<'sr> {

    fn default() -> Self {
        Self {
            midi_programs_have_changed: false,
            learn_param:                -1,
            learn_custom:               -1,
            channel_state: Default::default(),
            phantom:       Default::default(),
        }
    }
}

impl MIDIUnit<'sr> {
    pub fn nrpn_last(&self, channel: u8) -> bool {
        self.channel_state[channel as usize].nrpn_last
    }

    pub fn load_midi_controllers(&mut self) {
        println!("to be decided");
    }

    pub fn save_midi_controllers(&mut self) {
        println!("to be decided");
    }
}


