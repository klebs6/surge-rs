crate::ix!();

impl SurgeVoice {

    pub fn init_portasrc(&mut self, cfg: &VoiceConstructor) {

        let voice_runtime = cfg.voice_runtime.borrow();

        let polymode        = cfg.polymode;
        let portamento      = voice_runtime.portamento;
        let portamento_min  = voice_runtime.portamento_min;

        let last_key: i32   = self.mpe_unit.get_lastkey();
        let pitch           = self.state.get_pitch();

        self.state.set_portasrc_key(
            polymode, 
            (portamento - portamento_min).abs() < f32::EPSILON,
            last_key, 
            pitch as f64
        );
    }
}
