crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum RingModulatorParam {
    CarrierShape,
    CarrierFreq,
    UnisonDetune,
    UnisonVoices,
    DiodeForwardBias,
    DiodeLinearRegion,
    LowCut,
    HighCut,
    Mix,
    ReturnLevel,
}

impl GetControlGroup for RingModulatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for RingModulatorParam {

    fn control_type(&self) -> ControlType {
        match self {
            RingModulatorParam::CarrierShape      => ControlType::SineOscMode,
            RingModulatorParam::CarrierFreq       => ControlType::FlangerPitch,
            RingModulatorParam::UnisonDetune      => ControlType::OscSpread,
            RingModulatorParam::UnisonVoices      => ControlType::OscCount,
            RingModulatorParam::DiodeForwardBias  => ControlType::Percent,
            RingModulatorParam::DiodeLinearRegion => ControlType::Percent,
            RingModulatorParam::LowCut            => ControlType::FreqAudible,
            RingModulatorParam::HighCut           => ControlType::FreqAudible,
            RingModulatorParam::Mix               => ControlType::Percent,
            RingModulatorParam::ReturnLevel       => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for RingModulatorParam {

    fn default_value(&self) -> PData {
        match self {
            RingModulatorParam::CarrierShape      => PData::Int(0),
            RingModulatorParam::CarrierFreq       => PData::Float(60.0),
            RingModulatorParam::UnisonDetune      => PData::Float(0.2),
            RingModulatorParam::UnisonVoices      => PData::Int(1),
            RingModulatorParam::DiodeForwardBias  => PData::Float(0.3),
            RingModulatorParam::DiodeLinearRegion => PData::Float(0.7),
            RingModulatorParam::LowCut            => self.min_value(),
            RingModulatorParam::HighCut           => self.max_value(),
            RingModulatorParam::Mix               => PData::Float(1.0),
            RingModulatorParam::ReturnLevel       => PData::Float(1.0),
        }
    }
}

impl GetMinParameterValue for RingModulatorParam {

    fn min_value(&self) -> PData {
        match self {
            RingModulatorParam::CarrierShape      => PData::Int(0),
            RingModulatorParam::CarrierFreq       => PData::Float(0.01),//TODO
            RingModulatorParam::UnisonDetune      => PData::Float(1.0),//TODO
            RingModulatorParam::UnisonVoices      => PData::Int(1),
            RingModulatorParam::DiodeForwardBias  => PData::Float(0.0),
            RingModulatorParam::DiodeLinearRegion => PData::Float(0.0),
            RingModulatorParam::LowCut            => PData::Float(0.0),
            RingModulatorParam::HighCut           => PData::Float(0.0),
            RingModulatorParam::Mix               => PData::Float(0.0),
            RingModulatorParam::ReturnLevel       => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for RingModulatorParam {

    fn max_value(&self) -> PData {
        match self {
            RingModulatorParam::CarrierShape      => PData::Int(0),//TODO
            RingModulatorParam::CarrierFreq       => PData::Float(0.0),
            RingModulatorParam::UnisonDetune      => PData::Float(0.0),
            RingModulatorParam::UnisonVoices      => PData::Int(RINGMOD_MAX_UNISON),
            RingModulatorParam::DiodeForwardBias  => PData::Float(1.0),
            RingModulatorParam::DiodeLinearRegion => PData::Float(1.0),
            RingModulatorParam::LowCut            => PData::Float(0.0),//TODO
            RingModulatorParam::HighCut           => PData::Float(0.0),//TODO
            RingModulatorParam::Mix               => PData::Float(1.0),
            RingModulatorParam::ReturnLevel       => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for RingModulatorParam {

    fn value_type(&self) -> ValType {
        match self {
            RingModulatorParam::CarrierShape      => ValType::VtInt,
            RingModulatorParam::CarrierFreq       => ValType::VtFloat,
            RingModulatorParam::UnisonDetune      => ValType::VtFloat,
            RingModulatorParam::UnisonVoices      => ValType::VtInt,
            RingModulatorParam::DiodeForwardBias  => ValType::VtFloat,
            RingModulatorParam::DiodeLinearRegion => ValType::VtFloat,
            RingModulatorParam::LowCut            => ValType::VtFloat,
            RingModulatorParam::HighCut           => ValType::VtFloat,
            RingModulatorParam::Mix               => ValType::VtFloat,
            RingModulatorParam::ReturnLevel       => ValType::VtFloat,
        }
    }
}

impl GetMoverate for RingModulatorParam {

    fn moverate(&self) -> f32 {
        match self {
            RingModulatorParam::CarrierShape      => 1.0,
            RingModulatorParam::CarrierFreq       => 1.0,
            RingModulatorParam::UnisonDetune      => 1.0,
            RingModulatorParam::UnisonVoices      => 1.0,
            RingModulatorParam::DiodeForwardBias  => 1.0,
            RingModulatorParam::DiodeLinearRegion => 1.0,
            RingModulatorParam::LowCut            => 1.0,
            RingModulatorParam::HighCut           => 1.0,
            RingModulatorParam::Mix               => 1.0,
            RingModulatorParam::ReturnLevel       => 1.0,
        }
    }
}

impl CheckIfModulateable for RingModulatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}
