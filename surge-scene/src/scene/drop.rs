ix!();

use crate::{
    SurgeScene,
};

impl SurgeScene {

    pub fn free_voice(&mut self, idx: usize) {

        //TODO: does this work? 
        self.voices[idx].borrow_mut().free_allocated_elements();

        self.voices.remove(idx);
    }

    pub fn free(&mut self) {

        let mut to_free = vec![];

        for (idx, _item) in self.voices.iter_mut().enumerate() {
            to_free.push(idx);
        }

        for idx in to_free.iter() {
            self.free_voice(*idx);
        }

        self.voices.clear();
    }

    pub fn purge_holdbuffer(&mut self) {

        let channel_state0 = self.midi_unit.channel_state_ptr(0) as *const MidiChannelState;

        let release_buffer = unsafe { 
            self.hold_pedal_unit.purge(channel_state0)
        };

        for (channel, key) in release_buffer.iter() {
            self.release_note_post_hold_check(*channel as u8, *key as u8, 127, None);
        }
    }

    #[inline] pub fn clear_channel_state(&mut self, 
        channel: u8, 
        key: u8) 
    {
        self.midi_unit.clear_keystate(channel,key);
    }
}

//TODO eliminate this
impl Drop for SurgeScene  {

    fn drop(&mut self) {
        self.modsources[ModSource::ModWheel]          = None;
        self.modsources[ModSource::ChannelAfterTouch] = None;
        self.modsources[ModSource::PitchBend]         = None;

        for i in 0_usize..N_LFOS_PER_SCENE {
            let ms = ModSource::scene_lfo(i);
            self.modsources[ms] = None;
        }
    }
}
