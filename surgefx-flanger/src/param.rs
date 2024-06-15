crate::ix!();

enhanced_enum![FlangerParam {
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
}];

rt![FlangerParam];

impl ParameterInterface for FlangerParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 

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

    fn modulateable(&self) -> bool {
        //true for all
        true
    }

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

impl FlangerParam {
    #[inline] pub fn new_runtime() -> FlangerParamArrayRT {
        FlangerParamArrayRT::new_with(|x| match x {
            FlangerParam::Rate           => FlangerParamRT::new(FlangerParam::Rate          ),
            FlangerParam::Depth          => FlangerParamRT::new(FlangerParam::Depth         ),
            FlangerParam::Mode           => FlangerParamRT::new(FlangerParam::Mode          ),
            FlangerParam::Voices         => FlangerParamRT::new(FlangerParam::Voices        ),
            FlangerParam::VoiceZeroPitch => FlangerParamRT::new(FlangerParam::VoiceZeroPitch),
            FlangerParam::VoiceDetune    => FlangerParamRT::new(FlangerParam::VoiceDetune   ),
            FlangerParam::VoiceChord     => FlangerParamRT::new(FlangerParam::VoiceChord    ),
            FlangerParam::Feedback       => FlangerParamRT::new(FlangerParam::Feedback      ),
            FlangerParam::FbLFDamping    => FlangerParamRT::new(FlangerParam::FbLFDamping   ),
            FlangerParam::Gain           => FlangerParamRT::new(FlangerParam::Gain          ),
            FlangerParam::StereoWidth    => FlangerParamRT::new(FlangerParam::StereoWidth   ),
            FlangerParam::Mix            => FlangerParamRT::new(FlangerParam::Mix           ),
            FlangerParam::ReturnLevel    => FlangerParamRT::new(FlangerParam::ReturnLevel   ),
        })
    }
}
