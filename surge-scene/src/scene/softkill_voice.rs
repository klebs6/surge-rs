ix!();

use crate::*;

impl SurgeScene {

    pub fn softkill_voice(&mut self) {

        let mut max_playing:  Option<Rc<RefCell<SurgeVoice>>> = None;
        let mut max_released: Option<Rc<RefCell<SurgeVoice>>> = None;

        let mut max_age = 0;
        let mut max_age_release = 0;

        let voices = &mut self.voices;

        for voice in voices.iter_mut() 
        {
            if voice.borrow().state.gate 
            {
                if voice.borrow().age > max_age 
                {
                    max_age = voice.borrow().age;
                    max_playing = Some(voice.clone());
                }

            } else if !voice.borrow().state.uberrelease  && voice.borrow().age_release > max_age_release {
                max_age_release = voice.borrow().age_release;
                max_released = Some(voice.clone());
            }
        }

        if max_age_release != 0 {
            max_released.unwrap().borrow_mut().uber_release();
        } else if max_age != 0 {
            max_playing.unwrap().borrow_mut().uber_release();
        }
    }
}
