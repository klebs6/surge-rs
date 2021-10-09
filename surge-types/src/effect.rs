ix!();
use crate::NumberOfBlocks;

#[derive(Debug,Copy,Clone)]
pub enum Ringout {
    On {counter: NumberOfBlocks, decay: NumberOfBlocks},
    Off,
}

impl Ringout {
    pub fn blocks(decay: NumberOfBlocks) -> Self {
        Ringout::On {
            counter: 0,
            decay,
        }
    }
}

enhanced_enum![
    EffectType {
        DualDelay,
        Eq3Band,
        Phaser,
        RotarySpeaker,
        Distortion,
        Reverb1,
        Reverb2,
        Freqshift,
        Conditioner,
        Chorus,
        Vocoder,
        Flanger,
    }
];

enhanced_enum![
    FxBypassType {
        AllFX,
        NoSendFX,
        SceneFXOnly,
        AllFXOff,
    }
];

enhanced_enum![FlangerWave {
    Tri,
    Sin,
    Saw,
    SampleAndHold,
}];

enhanced_enum![FlangerType {
    Unison,
    Doppler,
    Arp,
}];

impl FlangerType {
    pub fn feedback_scale(&self) -> f32 {
        match self {
            FlangerType::Unison  => 0.4,
            FlangerType::Doppler => 0.6,
            FlangerType::Arp     => 0.9,
        }
    }
}

