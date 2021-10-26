ix!();

use crate::{
    SurgeScene,
};

impl SurgeScene {

    #[allow(unreachable_code)]
    pub fn play_voice_portamento(&mut self, 
        channel: u8, 
        key: u8, 
        velocity: u8, 
        detune: u8) 
    {
        let voices = &mut self.voices;

        let mut found_one: bool = false;

        for voice in voices.iter_mut() {

            if voice.state.gate {

                voice.legato(key as i32, velocity as i32, detune as usize);

                found_one = true;

                if self.mpe_unit.enabled().0 {
                    // This voice was created on a channel but is being 
                    // legato held to another channel
                    // so it needs to borrow the channel and 
                    // self.midi_unit.channel_state. Obviously this can only
                    // happen in MPE mode.
                    voice.state.channel = channel as i32;

                    voice.state.voice_channel_state = self.midi_unit.channel_state_ptr(channel as u8);
                }

                break;

            } else {

                // make this optional for poly legato
                voice.uber_release(); 
            }
        }

        if !found_one {

            let cfg = self.voice_constructor(channel,key,velocity,detune);

            if let Some(nvoice) = self.get_unused_voice() {

                *nvoice = Some(SurgeVoice::new(cfg));

                //todo: how do we do this?
                //self.voices.push(nvoice.as_mut().unwrap());

            }
        }
    }
}
