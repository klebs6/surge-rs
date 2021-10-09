ix!();

use crate::{
    SurgeScene,
};

impl SurgeScene<'sr> {

    pub fn softkill_voice(&mut self) {

        let mut max_playing:  Option<&mut SurgeVoice> = None;
        let mut max_released: Option<&mut SurgeVoice> = None;

        let mut max_age = 0;
        let mut max_age_release = 0;

        let voices = &mut self.voices;

        for voice in voices.iter_mut() 
        {
            if voice.state.gate 
            {
                if voice.age > max_age 
                {
                    max_age = voice.age;
                    max_playing = Some(&mut *voice);
                }

            } else if !voice.state.uberrelease  && voice.age_release > max_age_release {
                max_age_release = voice.age_release;
                max_released = Some(&mut *voice);
            }
        }

        if max_age_release != 0 {
            max_released.unwrap().uber_release();

        } else if max_age != 0 {
            max_playing.unwrap().uber_release();
        }

    }
}
