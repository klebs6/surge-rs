ix!();

enhanced_enum![
    ModSource {
        //order matters for comparison functions
        Original,
        Velocity,
        KeyTrack,
        PolyphonicAfterTouch,
        ChannelAfterTouch,
        PitchBend,
        ModWheel,

        Ctrl1,
        Ctrl2,
        Ctrl3,
        Ctrl4,
        Ctrl5,
        Ctrl6,
        Ctrl7,
        Ctrl8,

        AmpEg,
        FilterEg,

        VoiceLfo1,
        VoiceLfo2,
        VoiceLfo3,
        VoiceLfo4,
        VoiceLfo5,
        VoiceLfo6,

        SceneLfo1,
        SceneLfo2,
        SceneLfo3,
        SceneLfo4,
        SceneLfo5,
        SceneLfo6,

        Timbre,
        ReleaseVelocity,
    }
];

impl ModSource {
    pub fn ctrl(idx: usize) -> Self { 
        match idx {
            0 => ModSource::Ctrl1,
            1 => ModSource::Ctrl2,
            2 => ModSource::Ctrl3,
            3 => ModSource::Ctrl4,
            4 => ModSource::Ctrl5,
            5 => ModSource::Ctrl6,
            6 => ModSource::Ctrl7,
            7 => ModSource::Ctrl8,
            _ => unreachable!(),
        }
    }
    pub fn scene_lfo(idx: usize) -> Self { 
        match idx {
            0 => ModSource::SceneLfo1,
            1 => ModSource::SceneLfo2,
            2 => ModSource::SceneLfo3,
            3 => ModSource::SceneLfo4,
            4 => ModSource::SceneLfo5,
            5 => ModSource::SceneLfo6,
            _ => unreachable!(),
        }
    }
    pub fn voice_lfo(idx: usize) -> Self { 
        match idx {
            0 => ModSource::VoiceLfo1,
            1 => ModSource::VoiceLfo2,
            2 => ModSource::VoiceLfo3,
            3 => ModSource::VoiceLfo4,
            4 => ModSource::VoiceLfo5,
            5 => ModSource::VoiceLfo6,
            _ => unreachable!(),
        }
    }

    pub fn can_modulate_monophonic_target(&self) -> bool { 
        (self.is_scenelevel()) || (*self == ModSource::ChannelAfterTouch) 
    }

    pub fn is_custom_controller(&self) -> bool { 
        (*self >= ModSource::Ctrl1) && (*self <= ModSource::Ctrl8)
    }

    pub fn is_envelope(&self) -> bool { 
        (*self == ModSource::AmpEg) || (*self == ModSource::FilterEg)
    }

    pub fn is_lfo(&self) -> bool { 
        (*self >= ModSource::VoiceLfo1) && (*self <= ModSource::SceneLfo6)
    }

    pub fn can_modulate_modulators(&self) -> bool { 
        (*self != ModSource::AmpEg) && (*self != ModSource::FilterEg)
    }

    pub fn is_voice_modulator(&self) -> bool { 
        !(
            (*self >= ModSource::SceneLfo1) && (*self <= ModSource::SceneLfo6)
        ) 
    }

    pub fn can_modulate_voice_modulators(&self)  -> bool { 
        (*self <= ModSource::Ctrl8) || (*self == ModSource::Timbre) }

    pub fn is_scenelevel(&self) -> bool {
        match &self {
            ModSource::Original             => true,
            ModSource::Velocity             => false,
            ModSource::KeyTrack             => false,
            ModSource::PolyphonicAfterTouch => false,
            ModSource::ChannelAfterTouch    => true,
            ModSource::PitchBend            => true,
            ModSource::ModWheel             => true,
            ModSource::Ctrl1                => true,
            ModSource::Ctrl2                => true,
            ModSource::Ctrl3                => true,
            ModSource::Ctrl4                => true,
            ModSource::Ctrl5                => true,
            ModSource::Ctrl6                => true,
            ModSource::Ctrl7                => true,
            ModSource::Ctrl8                => true,
            ModSource::AmpEg                => false,
            ModSource::FilterEg             => false,
            ModSource::VoiceLfo1            => false,
            ModSource::VoiceLfo2            => false,
            ModSource::VoiceLfo3            => false,
            ModSource::VoiceLfo4            => false,
            ModSource::VoiceLfo5            => false,
            ModSource::VoiceLfo6            => false,
            ModSource::SceneLfo1            => true,
            ModSource::SceneLfo2            => true,
            ModSource::SceneLfo3            => true,
            ModSource::SceneLfo4            => true,
            ModSource::SceneLfo5            => true,
            ModSource::SceneLfo6            => true,
            ModSource::Timbre               => false,
            ModSource::ReleaseVelocity      => false,
        }
    }
}
