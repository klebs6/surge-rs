ix!();

use crate::*;

#[derive(Debug)]
pub struct FlangerMode(pub FlangerType, pub FlangerWave);

impl FlangerParam {
    pub fn category(&self) -> &'static str {
        match self {
            FlangerParam::Rate | FlangerParam::Depth | FlangerParam::Mode => "Basic Control",
            FlangerParam::Voices
            | FlangerParam::VoiceZeroPitch
            | FlangerParam::VoiceDetune
            | FlangerParam::VoiceChord => "Voices",
            FlangerParam::Feedback
            | FlangerParam::FbLFDamping
            | FlangerParam::Gain
            | FlangerParam::StereoWidth
            | FlangerParam::ReturnLevel
            | FlangerParam::Mix => "Feedback and eq",
        }
    }
    pub fn describe(&self) -> &'static str {
        match self {
            FlangerParam::Rate           => "How quickly the oscillations happen",
            FlangerParam::Depth          => "How extreme the modulation of the delay is",
            FlangerParam::Mode           => "flange, phase-inverse-flange, arepeggio, vibrato",
            FlangerParam::Voices         => "how many delay lines",
            FlangerParam::VoiceZeroPitch => "tune the first max delay line to this (M = sr / f)",
            FlangerParam::VoiceDetune    => "spread in cents among the delay lines",
            FlangerParam::VoiceChord     => "tuning for the voices as a chord in tuning space",
            FlangerParam::Feedback       => "how much the output feeds back into the filters",
            FlangerParam::FbLFDamping    => "how much low pass damping in the feedback mechanism",
            FlangerParam::Gain           => "How much gain before we run into the final clipper",
            FlangerParam::ReturnLevel    => "scales the effect global return value",
            FlangerParam::StereoWidth => {
                "how much to pan the delay lines ( 0 -> all even; 1 -> full spread)"
            }
            FlangerParam::Mix => "how much we add the comb into the mix",
        }
    }
}

