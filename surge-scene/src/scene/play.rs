ix!();

use crate::{
    SurgeScene,
};

impl SurgeScene {

    #[inline] pub fn maybe_play_note(&mut self, play: bool, channel: u8) {

        let polymode = self.get_polymode();

        if let (true, true, true) = (play, polymode == PolyMode::LatchMonophonic, self.voices.is_empty()) 
        {
            self.play_note(channel,60, 100, 0);
        }
    }

    pub fn play_note(&mut self, channel: u8, key: u8, velocity: u8, detune: u8) {

        self.play_voice(channel as i32, key, velocity, detune);

        self.midi_unit.set_keystate(channel, key, velocity);
        self.midi_unit.set_lastdetune(channel, key, detune);

        // OK so why is there hold stuff here? This is play not release.
        // Well if you release a key with the pedal down it goes into the
        // 'release me later' buffer. If you press the key again it stays there
        // so even with the key held, you end up releasing it when you pedal. 
        // 
        // Or: NoteOn / Pedal On / Note Off / Note On / Pedal Off should leave the note ringing
        // 
        // and right now it doesn't
        //
        let mut no_hold: bool = !self.midi_unit.hold(channel);

        if self.mpe_unit.enabled().0 {

            let hold = self.midi_unit.hold(0);

            no_hold = no_hold && ! hold;
        }

        if !no_hold {
            for _s in 0..2 {
                self.hold_pedal_unit.reset(channel,key);
            }
        }
    }
}
