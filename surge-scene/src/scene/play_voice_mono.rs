crate::ix!();

impl SurgeScene {

    pub fn play_voice_mono(&mut self, 
        channel: u8, 
        key: u8, 
        velocity: u8, 
        detune: u8) 
    {
        let voices = &mut self.voices;

        let mut glide: bool = false;

        for voice in voices.iter_mut() {

            if voice.borrow().state.gate {
                glide = true;
            }

            voice.borrow_mut().uber_release();
        }

        let mut do_set_last_key = false;

        let cfg = self.voice_constructor(channel,key,velocity,detune);

        if let Some(nvoice) = self.get_unused_voice() {

            do_set_last_key = true;

            *nvoice = Some(SurgeVoice::new(cfg));
        }

        if do_set_last_key 
            && !glide 
                && (self.get_polymode() == PolyMode::MonoFingeredPortamento) 
        {
            self.mpe_unit.set_lastkey(key as i32);
        }
    }
}
