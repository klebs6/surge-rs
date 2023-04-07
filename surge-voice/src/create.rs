crate::ix!();

pub fn some_new_boxed_lfo(
    time_unit: TimeUnitHandle,
    tables:    TablesHandle
) -> Option<Box<ModulationSource>> 
{
    let lfo = Lfo::new(
        time_unit.clone(), 
        tables.clone()
    );

    let ms  = ModulationSource::Lfo(lfo);

    Some(Box::new(ms))
}

pub fn some_new_boxed_controller(
    srunit:    SampleRateHandle
) -> Option<Box<ModulationSource>> 
{
    let cms = ControllerModulationSource::new(srunit.clone());

    let ms  = ModulationSource::ControllerModulationSource(cms);

    Some(Box::new(ms))
}

pub fn some_new_boxed_envelope(
    time_unit: TimeUnitHandle,
    tables:    TablesHandle,
    srunit:    SampleRateHandle,
) -> Option<Box<ModulationSource>> 
{
    let env = AdsrEnvelope::new(
        time_unit.clone(), 
        tables.clone(), 
        srunit.clone()
    );

    let ms = ModulationSource::AdsrEnvelope(env);

    Some(Box::new(ms))
}

pub fn create_voice_modsources(
    time_unit: TimeUnitHandle,
    tables:    TablesHandle,
    srunit:    SampleRateHandle,
) -> VoiceModSourceArray 
{
    let mut x = ModSourceArray::<Option<Box<ModulationSource>>>::new_with(|_x| {
        None
    });
    x[ModSource::VoiceLfo1]            = some_new_boxed_lfo(time_unit.clone(),tables.clone());
    x[ModSource::VoiceLfo2]            = some_new_boxed_lfo(time_unit.clone(),tables.clone());
    x[ModSource::VoiceLfo3]            = some_new_boxed_lfo(time_unit.clone(),tables.clone());
    x[ModSource::VoiceLfo4]            = some_new_boxed_lfo(time_unit.clone(),tables.clone());
    x[ModSource::VoiceLfo5]            = some_new_boxed_lfo(time_unit.clone(),tables.clone());
    x[ModSource::VoiceLfo6]            = some_new_boxed_lfo(time_unit.clone(),tables.clone());
    x[ModSource::ReleaseVelocity]      = some_new_boxed_controller(srunit.clone());
    x[ModSource::PolyphonicAfterTouch] = some_new_boxed_controller(srunit.clone());
    x[ModSource::Velocity]             = some_new_boxed_controller(srunit.clone());
    x[ModSource::KeyTrack]             = some_new_boxed_controller(srunit.clone());
    x[ModSource::ChannelAfterTouch]    = some_new_boxed_controller(srunit.clone());
    x[ModSource::Timbre]               = some_new_boxed_controller(srunit.clone());
    x[ModSource::AmpEg]                = some_new_boxed_envelope(time_unit.clone(),tables.clone(),srunit.clone());
    x[ModSource::FilterEg]             = some_new_boxed_envelope(time_unit.clone(),tables.clone(),srunit.clone());
    x
}

pub fn create_voice_osclevels() -> VoiceOscLevels {

    LagEntryArray::<LipolPs>::new_with(|x| match x {
        LagEntry::Osc1   => LipolPs::new(),
        LagEntry::Osc2   => LipolPs::new(),
        LagEntry::Osc3   => LipolPs::new(),
        LagEntry::Noise  => LipolPs::new(),
        LagEntry::Ring12 => LipolPs::new(),
        LagEntry::Ring23 => LipolPs::new(),
        LagEntry::Pfg    => LipolPs::new(),
    })
}

pub fn create_voice_oscillators( tuner: TunerHandle) 
-> MaybeVoiceOscillators 
{
    [
        Some(Box::new(SineWaveOscillator::new(tuner.clone()))),
        Some(Box::new(SineWaveOscillator::new(tuner.clone()))),
        Some(Box::new(SineWaveOscillator::new(tuner.clone()))),
    ]
}
