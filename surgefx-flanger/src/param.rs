crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum FlangerParam {
    Rate,
    Depth,
    Mode,
    Voices,
    VoiceZeroPitch,
    VoiceDetune,
    VoiceChord,
    Feedback,
    FbLFDamping,
    Gain,
    StereoWidth,
    Mix,
    ReturnLevel,
}

impl_trait_defaults!{
    FlangerParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetModulation,
    GetSnap,
    SetModulation,
    GetExtendedValue,
    GetExtendRange,
}

impl CheckIfAbsolute for FlangerParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for FlangerParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for FlangerParam {

    fn control_type(&self) -> ControlType {
        match self {
            FlangerParam::Rate           => ControlType::LfoRate,
            FlangerParam::Depth          => ControlType::Percent,
            FlangerParam::Mode           => ControlType::FlangerMode,
            FlangerParam::Voices         => ControlType::FlangerVoices,
            FlangerParam::VoiceZeroPitch => ControlType::FlangerPitch,
            FlangerParam::VoiceDetune    => ControlType::Percent,
            FlangerParam::VoiceChord     => ControlType::Percent,
            FlangerParam::Feedback       => ControlType::Percent,
            FlangerParam::FbLFDamping    => ControlType::Percent,
            FlangerParam::Gain           => ControlType::PercentBidirectional,
            FlangerParam::StereoWidth    => ControlType::Percent,
            FlangerParam::Mix            => ControlType::PercentBidirectional,
            FlangerParam::ReturnLevel    => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for FlangerParam {

    fn default_value(&self) -> PData {
        match self {
            FlangerParam::Rate           => PData::Float(-2.0),
            FlangerParam::Depth          => PData::Float(1.0),
            FlangerParam::Mode           => PData::Int(0),
            FlangerParam::Voices         => PData::Int(4),
            FlangerParam::VoiceZeroPitch => PData::Float(60.0),
            FlangerParam::VoiceDetune    => PData::Float(0.0),
            FlangerParam::VoiceChord     => PData::Float(0.0),
            FlangerParam::Feedback       => PData::Float(0.0),
            FlangerParam::FbLFDamping    => PData::Float(0.1),
            FlangerParam::Gain           => PData::Float(0.0),
            FlangerParam::StereoWidth    => PData::Float(0.0),
            FlangerParam::Mix            => PData::Float(0.8),
            FlangerParam::ReturnLevel    => PData::Float(0.5),
        }
    }
}

impl CheckIfModulateable for FlangerParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for FlangerParam {

    fn min_value(&self) -> PData {
        match self {
            FlangerParam::Rate           => PData::Float(-7.0),
            FlangerParam::Depth          => PData::Float(0.0), 
            FlangerParam::Mode           => PData::Int(0),     
            FlangerParam::Voices         => PData::Int(1), 
            FlangerParam::VoiceZeroPitch => PData::Float(0.0), 
            FlangerParam::VoiceDetune    => PData::Float(0.0), 
            FlangerParam::VoiceChord     => PData::Float(0.0), 
            FlangerParam::Feedback       => PData::Float(0.0), 
            FlangerParam::FbLFDamping    => PData::Float(0.0), 
            FlangerParam::Gain           => PData::Float(-1.0),
            FlangerParam::StereoWidth    => PData::Float(0.0), 
            FlangerParam::Mix            => PData::Float(-1.0),
            FlangerParam::ReturnLevel    => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for FlangerParam {

    fn max_value(&self) -> PData {
        match self {
            FlangerParam::Rate           => PData::Float(9.0),  
            FlangerParam::Depth          => PData::Float(1.0),  
            FlangerParam::Mode           => PData::Int(11),     
            FlangerParam::Voices         => PData::Int(4),  
            FlangerParam::VoiceZeroPitch => PData::Float(127.0),
            FlangerParam::VoiceDetune    => PData::Float(1.0),  
            FlangerParam::VoiceChord     => PData::Float(1.0),  
            FlangerParam::Feedback       => PData::Float(1.0),  
            FlangerParam::FbLFDamping    => PData::Float(1.0),  
            FlangerParam::Gain           => PData::Float(1.0),  
            FlangerParam::StereoWidth    => PData::Float(1.0),  
            FlangerParam::Mix            => PData::Float(1.0),  
            FlangerParam::ReturnLevel    => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for FlangerParam {

    fn value_type(&self) -> ValType {
        match self {
            FlangerParam::Rate           => ValType::VtFloat,
            FlangerParam::Depth          => ValType::VtFloat,
            FlangerParam::Mode           => ValType::VtInt,  
            FlangerParam::Voices         => ValType::VtInt,
            FlangerParam::VoiceZeroPitch => ValType::VtFloat,
            FlangerParam::VoiceDetune    => ValType::VtFloat,
            FlangerParam::VoiceChord     => ValType::VtFloat,
            FlangerParam::Feedback       => ValType::VtFloat,
            FlangerParam::FbLFDamping    => ValType::VtFloat,
            FlangerParam::Gain           => ValType::VtFloat,
            FlangerParam::StereoWidth    => ValType::VtFloat,
            FlangerParam::Mix            => ValType::VtFloat,
            FlangerParam::ReturnLevel    => ValType::VtFloat,
        }
    }
}

impl GetMoverate for FlangerParam {

    fn moverate(&self) -> f32 {
        match self {
            FlangerParam::Rate           => 0.33,
            FlangerParam::Depth          => 1.0, 
            FlangerParam::Mode           => 1.0, 
            FlangerParam::Voices         => 1.0, 
            FlangerParam::VoiceZeroPitch => 1.0, 
            FlangerParam::VoiceDetune    => 1.0, 
            FlangerParam::VoiceChord     => 1.0, 
            FlangerParam::Feedback       => 1.0, 
            FlangerParam::FbLFDamping    => 1.0, 
            FlangerParam::Gain           => 1.0, 
            FlangerParam::StereoWidth    => 1.0, 
            FlangerParam::Mix            => 1.0, 
            FlangerParam::ReturnLevel    => 1.0,
        }
    }
}
