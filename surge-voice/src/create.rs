ix!();

use crate::{
    LagEntry,
    VoiceModSourceArray,
    VoiceOscLevels,
    LagEntryArray,
    MaybeVoiceOscillators,
};

pub fn some_new_boxed_lfo<'sr>(
    time_unit: TimeUnitHandle<'sr>,
    tables:    TablesHandle<'sr>
) -> Option<Box<ModulationSource<'sr>>> 
{
    Some(
        Box::new(
            ModulationSource::Lfo(
                Lfo::new(time_unit.clone(), tables.clone())
            )
        )
    )
}

pub fn some_new_boxed_controller(
    srunit:    SampleRateHandle<'_>
) -> Option<Box<ModulationSource<'_>>> 
{
    Some(
        Box::new(
            ModulationSource::ControllerModulationSource(
                ControllerModulationSource::new(srunit.clone())
            )
        )
    )
}

pub fn some_new_boxed_envelope<'sr>(
    time_unit: TimeUnitHandle<'sr>,
    tables:    TablesHandle<'sr>,
    srunit:    SampleRateHandle<'sr>,
) -> Option<Box<ModulationSource<'sr>>> 
{
    Some(
        Box::new(
            ModulationSource::AdsrEnvelope(
                AdsrEnvelope::new(
                    time_unit.clone(),
                    tables.clone(),
                    srunit.clone()
                )
            )
        )
    )
}

pub fn create_voice_modsources<'sr>(
    time_unit: TimeUnitHandle<'sr>,
    tables:    TablesHandle<'sr>,
    srunit:    SampleRateHandle<'sr>,
) -> VoiceModSourceArray<'sr> 
{
    let mut x = ModSourceArray::<Option<Box<ModulationSource<'sr>>>>::new_with(|_x| {
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

pub fn create_voice_oscillators<'sr>( tuner: TunerHandle<'sr>) 
-> MaybeVoiceOscillators<'sr> 
{
    [
        Some(box SineWaveOscillator::new(tuner.clone())),
        Some(box SineWaveOscillator::new(tuner.clone())),
        Some(box SineWaveOscillator::new(tuner.clone())),
    ]
}
