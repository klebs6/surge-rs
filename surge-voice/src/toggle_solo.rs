crate::ix!();

#[derive(Clone)]
pub struct VoiceToggleSoloCfg {
    pub oscillator0_solo: bool,
    pub oscillator1_solo: bool,
    pub oscillator2_solo: bool,
    pub noise0_solo:      bool,
    pub ring0_solo:       bool,
    pub ring1_solo:       bool,
    pub oscillator0_mute: bool,
    pub oscillator1_mute: bool,
    pub oscillator2_mute: bool,
    pub noise_mute:       bool,
    pub ring0_mute:       bool,
    pub ring1_mute:       bool,
}

impl VoiceToggleSoloCfg {

    pub fn solo(&self) -> bool {
        let solo: bool = 
            self.oscillator0_solo 
            || self.oscillator1_solo 
            || self.oscillator2_solo 
            || self.noise0_solo 
            || self.ring0_solo 
            || self.ring1_solo;
        solo
    }
}

impl SurgeVoice {

    pub fn set_path(&mut self, 
        osc0:  bool, 
        osc1:  bool, 
        osc2:  bool, 
        ring0: bool, 
        ring1: bool, 
        noise: bool)
    {
        self.osc_enable   = [osc0, osc1, osc2];
        self.ring_enable  = [ring0, ring1];
        self.noise_enable = noise;
    }

    pub fn maybe_toggle_solo(&mut self, 
        cfg: VoiceRuntimeHandle) 
    {
        let cfg = cfg.borrow();

        //todo put this somewhere else?
        self.fm_mode = cfg.fm_cfg;

        if cfg.voice_toggle_solo_cfg.solo() {

            // inter-osc fm should be enabled only if 
            // carrier (osc 1) is soloed, in case any solos are active
            self.set_path(
                cfg.voice_toggle_solo_cfg.oscillator0_solo,
                cfg.voice_toggle_solo_cfg.oscillator1_solo,
                cfg.voice_toggle_solo_cfg.oscillator2_solo,
                cfg.voice_toggle_solo_cfg.ring0_solo,
                cfg.voice_toggle_solo_cfg.ring1_solo,
                cfg.voice_toggle_solo_cfg.noise0_solo,
            );

        } else {

            self.set_path(
                !cfg.voice_toggle_solo_cfg.oscillator0_mute,
                !cfg.voice_toggle_solo_cfg.oscillator1_mute,
                !cfg.voice_toggle_solo_cfg.oscillator2_mute,
                !cfg.voice_toggle_solo_cfg.ring0_mute,
                !cfg.voice_toggle_solo_cfg.ring1_mute,
                !cfg.voice_toggle_solo_cfg.noise_mute,
            );
        }
    }
}
