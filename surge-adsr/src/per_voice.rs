crate::ix!();

impl CheckIsModulationSourcePerVoice for AdsrEnvelope {

    /// indicates whether this modulation source
    /// is per-voice or per-note, returning `true`
    /// since the ADSR envelope is per-voice.
    ///
    const fn per_voice(&self) -> bool { 
        true 
    }
}
