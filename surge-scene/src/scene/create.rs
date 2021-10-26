ix!();

use crate::{
    SurgeScene,
};

impl SurgeScene {

    pub fn voice_toggle_solo_cfg(&self) -> VoiceToggleSoloCfg {
        VoiceToggleSoloCfg {
            oscillator0_solo: self.oscillator_solo(0),
            oscillator1_solo: self.oscillator_solo(1),
            oscillator2_solo: self.oscillator_solo(2),
            noise0_solo:      self.noise_solo(0),
            ring0_solo:       self.ring_solo(0),
            ring1_solo:       self.ring_solo(1),
            oscillator0_mute: self.oscillator_mute(0),
            oscillator1_mute: self.oscillator_mute(1),
            oscillator2_mute: self.oscillator_mute(2),
            noise_mute:       self.noise_mute(0),
            ring0_mute:       self.ring_mute(0),
            ring1_mute:       self.ring_mute(1),
        }
    }

    pub fn voice_constructor(&mut self, 
        channel: u8, 
        key: u8, 
        velocity: u8, 
        detune: u8) -> VoiceConstructor 
    {
        let mpe_main_channel: u8 = self.mpe_unit.get_mpe_main_channel(channel, key);

        VoiceConstructor {
            timeunit:                 self.timeunit.clone(),
            tables:                   self.tables.clone(),
            tuner:                    self.tuner.clone(),
            srunit:                   self.srunit.clone(),
            synth_in:                 self.synth_in.clone(),
            mpe_unit:                 self.mpe_unit.clone(),
            key:                      key as i32,
            velocity:                 velocity as i32,
            channel:                  channel as i32,
            detune:                   detune as f64,
            keystate:                 self.midi_unit.keystate_ptr(channel,key),
            main_channel_state:       self.midi_unit.channel_state_ptr(mpe_main_channel),
            voice_channel_state:      self.midi_unit.channel_state_ptr(channel),
            mpe_enabled:              self.mpe_unit.enabled(),
            fm_cfg:                   self.fm_cfg(),
            polymode:                 self.get_polymode(),
            filterunit_filtertype:    vec![
                self.filterunit_filtertype(0),
                self.filterunit_filtertype(1),
            ],
            filterunit_filtersubtype: vec![
                self.filterunit_filtersubtype(0),
                self.filterunit_filtersubtype(1),
            ],
            voice_toggle_solo_cfg:    self.voice_toggle_solo_cfg(),
            oscillator_type:          [
                self.oscillator_type(0),
                self.oscillator_type(1),
                self.oscillator_type(2),
            ],
            voice_runtime:            self.create_voice_runtime(),
        }
    }
}
