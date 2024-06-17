crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum VocoderParam {
    Gain,
    GateLevel,
    Rate,
    UnvoicedThreshold,
    Quality,
    NumBands,
    FreqLo,
    FreqHi,
    ModExpand,
    ModCenter,
    ReturnLevel
}

impl_trait_defaults!{
    VocoderParam;
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

impl CheckIfAbsolute for VocoderParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for VocoderParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for VocoderParam {

    fn control_type(&self) -> ControlType {
        match self {
            VocoderParam::Gain              => ControlType::Decibel,
            VocoderParam::GateLevel         => ControlType::DecibelAttenuationLarge,
            VocoderParam::Rate              => ControlType::Percent,
            VocoderParam::UnvoicedThreshold => ControlType::Decibel,
            VocoderParam::Quality           => ControlType::PercentBidirectional,
            VocoderParam::NumBands          => ControlType::VocoderBandcount,
            VocoderParam::FreqLo            => ControlType::FreqVocoderLow,
            VocoderParam::FreqHi            => ControlType::FreqVocoderHigh,
            VocoderParam::ModExpand         => ControlType::PercentBidirectional,
            VocoderParam::ModCenter         => ControlType::PercentBidirectional,
            VocoderParam::ReturnLevel       => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for VocoderParam {

    fn default_value(&self) -> PData {
        match self {
            VocoderParam::Gain              => PData::Float(0.0),
            VocoderParam::GateLevel         => PData::Float(-96.0),
            VocoderParam::Rate              => PData::Float(0.0),
            VocoderParam::UnvoicedThreshold => PData::Float(0.0),
            VocoderParam::Quality           => PData::Float(0.0),
            VocoderParam::NumBands          => PData::Int(N_VOCODER_BANDS as i32),
            VocoderParam::FreqLo            => PData::Float(vocoder_default_freq_low()),
            VocoderParam::FreqHi            => PData::Float(vocoder_default_freq_high()),
            VocoderParam::ModExpand         => PData::Float(0.0),
            VocoderParam::ModCenter         => PData::Float(0.0),
            VocoderParam::ReturnLevel       => PData::Float(0.5),
        }
    }
}

impl CheckIfModulateable for VocoderParam {

    fn modulateable(&self) -> bool {
        true
    }
}

impl GetMinParameterValue for VocoderParam {

    fn min_value(&self) -> PData {
        match self {
            VocoderParam::Gain              => PData::Float(-48.0),
            VocoderParam::GateLevel         => PData::Float(-96.0),
            VocoderParam::Rate              => PData::Float(0.0),  
            VocoderParam::UnvoicedThreshold => PData::Float(-48.0),
            VocoderParam::Quality           => PData::Float(-1.0), 
            VocoderParam::NumBands          => PData::Int(4),      
            VocoderParam::FreqLo            => PData::Float(-36.0),
            VocoderParam::FreqHi            => PData::Float(0.0),  
            VocoderParam::ModExpand         => PData::Float(-1.0), 
            VocoderParam::ModCenter         => PData::Float(-1.0), 
            VocoderParam::ReturnLevel       => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for VocoderParam {
    fn max_value(&self) -> PData {
        match self {
            VocoderParam::Gain              => PData::Float(48.0),
            VocoderParam::GateLevel         => PData::Float(0.0), 
            VocoderParam::Rate              => PData::Float(1.0), 
            VocoderParam::UnvoicedThreshold => PData::Float(48.0),
            VocoderParam::Quality           => PData::Float(1.0), 
            VocoderParam::NumBands          => PData::Int(20),    
            VocoderParam::FreqLo            => PData::Float(36.0),
            VocoderParam::FreqHi            => PData::Float(60.0),
            VocoderParam::ModExpand         => PData::Float(1.0), 
            VocoderParam::ModCenter         => PData::Float(1.0), 
            VocoderParam::ReturnLevel       => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for VocoderParam {
    fn value_type(&self) -> ValType {
        match self {
            VocoderParam::Gain              => ValType::VtFloat,
            VocoderParam::GateLevel         => ValType::VtFloat,
            VocoderParam::Rate              => ValType::VtFloat,
            VocoderParam::UnvoicedThreshold => ValType::VtFloat,
            VocoderParam::Quality           => ValType::VtFloat,
            VocoderParam::NumBands          => ValType::VtInt,  
            VocoderParam::FreqLo            => ValType::VtFloat,
            VocoderParam::FreqHi            => ValType::VtFloat,
            VocoderParam::ModExpand         => ValType::VtFloat,
            VocoderParam::ModCenter         => ValType::VtFloat,
            VocoderParam::ReturnLevel       => ValType::VtFloat,
        }
    }
}

impl GetMoverate for VocoderParam {
    fn moverate(&self) -> f32 {
        match self {
            VocoderParam::Gain              => 1.0,
            VocoderParam::GateLevel         => 1.0,
            VocoderParam::Rate              => 1.0,
            VocoderParam::UnvoicedThreshold => 1.0,
            VocoderParam::Quality           => 1.0,
            VocoderParam::NumBands          => 1.0,
            VocoderParam::FreqLo            => 1.0,
            VocoderParam::FreqHi            => 1.0,
            VocoderParam::ModExpand         => 1.0,
            VocoderParam::ModCenter         => 1.0,
            VocoderParam::ReturnLevel       => 1.0,
        }
    }
}
