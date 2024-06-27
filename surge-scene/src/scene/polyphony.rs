crate::ix!();

impl SurgeScene {

    pub fn play_voice_poly(
        &mut self, 
        channel:  u8, 
        key:      u8, 
        velocity: u8, 
        detune:   u8

    ) -> Result<(),SurgeError> {

        let cfg = self.voice_constructor(channel, key, velocity, detune);

        if let Some(nvoice) = self.get_unused_voice() {

            *nvoice = Some(SurgeVoice::new(cfg)?);

            todo!( "how do we do this?" );
            //self.voices.push(nvoice);
        }

        Ok(())
    }

    /// only allow 'margin' number of voices to be softkilled simultaneously
    pub fn enforce_polyphony_limit(&mut self, limit: i32, margin: i32) -> Result<(),SurgeError> {

        let num_scene_voices = self.voices.len();

        if (num_scene_voices as i32) > limit + margin {

            let mut excess_voices: i32 = surge_math::maxi(0, (num_scene_voices as i32) - limit + margin);

            let mut idx_to_free = vec![];

            for (idx, voice) in self.voices.iter_mut().enumerate() {

                if excess_voices < 1 {
                    break;
                }

                if voice.borrow().state.uberrelease {

                    excess_voices -= 1;

                    idx_to_free.push(idx);
                } 
            }

            for idx in idx_to_free.iter() {
                self.free_voice(*idx)?;
            }
        }

        Ok(())
    }
}
