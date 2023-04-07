crate::ix!();

enhanced_enum![
    AudioInputOscillatorParam {
        Input,
        Gain,
    }
];

rt![AudioInputOscillatorParam];

impl Param for AudioInputOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

    fn control_type(&self) -> ControlType {
        match self {
            AudioInputOscillatorParam::Input => ControlType::PercentBidirectional,
            AudioInputOscillatorParam::Gain => ControlType::Decibel,
        }
    }

    fn default_value(&self) -> PData {
        match self {
            AudioInputOscillatorParam::Input => PData::Float(0.0),
            AudioInputOscillatorParam::Gain  => PData::Float(0.0),
        }
    }

    fn modulateable(&self) -> bool {
        //true for all
        true
    }

    fn min_value(&self) -> PData {
        match self {
            AudioInputOscillatorParam::Input => PData::Float(-1.0), 
            AudioInputOscillatorParam::Gain  => PData::Float(-48.0),
        }
    }

    fn max_value(&self) -> PData {
        match self {
            AudioInputOscillatorParam::Input => PData::Float(1.0), 
            AudioInputOscillatorParam::Gain  => PData::Float(48.0),
        }
    }

    fn value_type(&self) -> ValType {
        match self {
            AudioInputOscillatorParam::Input => ValType::VtFloat,
            AudioInputOscillatorParam::Gain  => ValType::VtFloat,
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            AudioInputOscillatorParam::Input => 1.0,
            AudioInputOscillatorParam::Gain  => 1.0,
        }
    }
}

impl AudioInputOscillatorParam {

    #[inline] pub fn new_runtime() -> AudioInputOscillatorParamArrayRT {
        AudioInputOscillatorParamArrayRT::new_with(|x| match x {
            AudioInputOscillatorParam::Input => AudioInputOscillatorParamRT::new(AudioInputOscillatorParam::Input), 
            AudioInputOscillatorParam::Gain  => AudioInputOscillatorParamRT::new(AudioInputOscillatorParam::Gain), 
        })
    }
}
