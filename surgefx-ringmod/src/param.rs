ix!();

use crate::RINGMOD_MAX_UNISON;

enhanced_enum![
    RingModulatorParam {
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
];

rt![RingModulatorParam];

impl Param for RingModulatorParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
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
    fn modulateable(&self) -> bool {
        //true for all
        true
    }

}

impl RingModulatorParam {

    pub fn new_runtime() -> RingModulatorParamArrayRT {
        RingModulatorParamArrayRT::new_with(|x| match x {
            RingModulatorParam::CarrierShape      => RingModulatorParamRT::new(RingModulatorParam::CarrierShape),
            RingModulatorParam::CarrierFreq       => RingModulatorParamRT::new(RingModulatorParam::CarrierFreq),
            RingModulatorParam::UnisonDetune      => RingModulatorParamRT::new(RingModulatorParam::UnisonDetune),
            RingModulatorParam::UnisonVoices      => RingModulatorParamRT::new(RingModulatorParam::UnisonVoices),
            RingModulatorParam::DiodeForwardBias  => RingModulatorParamRT::new(RingModulatorParam::DiodeForwardBias),
            RingModulatorParam::DiodeLinearRegion => RingModulatorParamRT::new(RingModulatorParam::DiodeLinearRegion),
            RingModulatorParam::LowCut            => RingModulatorParamRT::new(RingModulatorParam::LowCut),
            RingModulatorParam::HighCut           => RingModulatorParamRT::new(RingModulatorParam::HighCut),
            RingModulatorParam::Mix               => RingModulatorParamRT::new(RingModulatorParam::Mix),
            RingModulatorParam::ReturnLevel       => RingModulatorParamRT::new(RingModulatorParam::ReturnLevel),
        })
    }
}
