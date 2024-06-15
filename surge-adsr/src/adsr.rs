crate::ix!();

/// The ADSR envelope describes the change of
/// a sound's amplitude over time. 
///
/// This is a commonly used component in
/// electronic music synthesis. 
///
/// It contains various fields that store
/// information about the envelope's state,
///
#[derive(Debug)]
#[modulation_source_control]
#[name("envelope")]
pub struct AdsrEnvelope {

    /// an array of ADSR parameter values
    ///
    params:        AdsrParamArrayRT,

    /// the current output of the envelope
    ///
    output:        f32,

    /// the current phase of the envelope
    ///
    phase:         f32,

    /// the sustain level of the envelope
    ///
    sustain:       f32,

    /// the current scale stage of the envelope
    ///
    scalestage:    f32,

    /// the current idle count of the envelope
    ///
    idlecount:     i32,

    /// the current state of the envelope
    ///
    envstate:      AdsrState,

    /// internal state variables used in the
    /// analog envelope processing
    _v_c1:         f32,
    _v_c1_delayed: f32,
    _discharge:    f32,

    /// a handle to a time unit object
    ///
    time_unit:     TimeUnitHandle,

    /// a handle to a tables object
    ///
    tables:        TablesHandle,

    /// a handle to a sample rate object
    ///
    srunit:        SampleRateHandle,

    /// indicates whether the envelope is enabled
    ///
    enabled:       bool
}

impl GetModulationSourceType for AdsrEnvelope {

    /// returns the type of modulation source, in
    /// this case `ModSrcType::Adsr`.
    ///
    const fn modulation_source_type(&self) -> ModSrcType
    {
        ModSrcType::Adsr
    }
}
