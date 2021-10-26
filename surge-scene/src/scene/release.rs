ix!();

use crate::{
    SurgeScene,
    ReleaseCfg,
    KeyRange,
};

impl SurgeScene {

    #[inline] pub fn hold(&self, channel: u8) -> bool {
        let mut no_hold: bool = 
            !self.midi_unit.hold(channel);

        if self.mpe_unit.enabled().0 {
            no_hold = no_hold && 
                !self.midi_unit.hold(0);
        }
        !no_hold
    }

    #[inline] pub fn maybe_release(&mut self, play: bool) {

        if self.release_if_latched
        {
            if !play || self.release_anyway 
            {
                self.free();
            }

            self.release_if_latched = false;
            self.release_anyway     = false;
        }
    }

    #[inline] pub fn set_release_if_latched(&mut self, val: bool) {
        self.release_if_latched = val;
        self.release_anyway     = false;
    }

    #[inline] pub fn release_note_post(&mut self, 
        channel: u8, 
        key: u8, 
        velocity: u8,
        keyrange: Option<KeyRange>) 
    {

        if self.hold(channel) {
            // hold pedal is down, add to bufffer
            self.hold_pedal_unit.push(channel, key);

        } else {
            self.release_note_post_hold_check(channel, key, velocity, keyrange);
        }
    }

    #[inline] pub fn release_note_post_hold_check(&mut self, 
        channel: u8, 
        key: u8, 
        velocity: u8, 
        keyrange: Option<KeyRange>) 
    {
        self.clear_channel_state(channel,key);

        for voice_idx in 0..self.voices.len() 
        {
            let cfg = self.get_release_voice_cfg(
                channel, 
                key, 
                velocity, 
                &keyrange
            );
            self.release_voice(voice_idx, cfg);
        }

        if self.get_non_released_voices() == 0 {

            for l in 0..N_LFOS_PER_SCENE {

                if let Some(ref mut src) = self.modsources[ModSource::scene_lfo(l)] {
                    src.release();
                }

            }
        }
    }

    pub fn release_note(&mut self, channel: u8, key: u8, velocity: u8, keyrange: Option<KeyRange>) 
    {
        for voice in self.voices.iter_mut() 
        {
            if voice.state.key == (key as i32) 
                && voice.state.channel == (channel as i32) 
            {
                voice.state.releasevelocity = velocity as i32;
            }
        }

        self.release_note_post(channel, key, velocity, keyrange);
    }


    pub fn release_voice(&mut self, 
        voice_idx: usize, 
        cfg: ReleaseCfg) 
    {
        let polymode = self.get_polymode();

        match polymode {
            PolyMode::Poly => {
                self.release_poly(voice_idx, cfg);
            },
            PolyMode::Mono | PolyMode::MonoFingeredPortamento | PolyMode::LatchMonophonic => {
                self.release_monophonic(voice_idx, cfg);
            },
            PolyMode::MonoSingleTriggerEG | PolyMode::MonoSingleTriggerFingeredPortamento => {
                self.release_single_triggered(voice_idx, cfg);
            },
        }
    }

    pub fn release_poly(&mut self, 
        voice_idx: usize, 
        cfg:       ReleaseCfg) 
    {
        let voice = &mut self.voices[voice_idx];

        if (voice.state.key, voice.state.channel) == 
            (cfg.key as i32, cfg.channel as i32) 
        {
            voice.release();
        }
    }
}
