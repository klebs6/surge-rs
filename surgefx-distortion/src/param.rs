crate::ix!();

//TODO: these are maybe not the best ones
//we have six filters, a drive, and an outgain
//find parameters that fit these blocks
enhanced_enum![
    DistortionParam {
        PreGain,
        PreFreq,
        PreBandwidth,
        PreHighCut,
        Drive,
        Feedback,
        PostGain,
        PostFreq,
        PostBandwidth,
        PostHighCut,
        OutGain,
        Waveshaper,
        ReturnLevel,
    }
];

rt![DistortionParam];

impl Param for DistortionParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
    fn control_type(&self) -> ControlType {
        match self {
            DistortionParam::PreGain       => ControlType::DecibelExtendable       , 
            DistortionParam::PreFreq       => ControlType::FreqAudible             , 
            DistortionParam::PreBandwidth  => ControlType::Bandwidth               , 
            DistortionParam::PreHighCut    => ControlType::FreqAudible             , 
            DistortionParam::Drive         => ControlType::DecibelNarrowExtendable , 
            DistortionParam::Feedback      => ControlType::PercentBidirectional    , 
            DistortionParam::PostGain      => ControlType::DecibelExtendable       , 
            DistortionParam::PostFreq      => ControlType::FreqAudible             , 
            DistortionParam::PostBandwidth => ControlType::Bandwidth               , 
            DistortionParam::PostHighCut   => ControlType::FreqAudible             , 
            DistortionParam::OutGain       => ControlType::DecibelNarrow           , 
            DistortionParam::Waveshaper    => ControlType::DistortionWaveshape     , 
            DistortionParam::ReturnLevel   => ControlType::Percent, 
        }
    }
    fn default_value(&self) -> PData {
        match self {
            DistortionParam::PreGain       => PData::Float(0.0), 
            DistortionParam::PreFreq       => PData::Float(0.0), 
            DistortionParam::PreBandwidth  => PData::Float(2.0), 
            DistortionParam::PreHighCut    => PData::Float(0.0), 
            DistortionParam::Drive         => PData::Float(0.0), 
            DistortionParam::Feedback      => PData::Float(0.0), 
            DistortionParam::PostGain      => PData::Float(0.0), 
            DistortionParam::PostFreq      => PData::Float(0.0), 
            DistortionParam::PostBandwidth => PData::Float(2.0), 
            DistortionParam::PostHighCut   => PData::Float(0.0), 
            DistortionParam::OutGain       => PData::Float(0.0), 
            DistortionParam::Waveshaper    => PData::Int(0), 
            DistortionParam::ReturnLevel   => PData::Float(0.5), 
        }
    }
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
    fn min_value(&self) -> PData {
        match self {
            DistortionParam::PreGain       => PData::Float(-48.0),
            DistortionParam::PreFreq       => PData::Float(-60.0),
            DistortionParam::PreBandwidth  => PData::Float(0.0),  
            DistortionParam::PreHighCut    => PData::Float(-60.0),
            DistortionParam::Drive         => PData::Float(-24.0),
            DistortionParam::Feedback      => PData::Float(-1.0), 
            DistortionParam::PostGain      => PData::Float(-48.0),
            DistortionParam::PostFreq      => PData::Float(-60.0),
            DistortionParam::PostBandwidth => PData::Float(0.0),  
            DistortionParam::PostHighCut   => PData::Float(-60.0),
            DistortionParam::OutGain       => PData::Float(-24.0),
            DistortionParam::Waveshaper    => PData::Int(0),      
            DistortionParam::ReturnLevel   => PData::Float(0.0), 
        }
    }
    fn max_value(&self) -> PData {
        match self {
            DistortionParam::PreGain       => PData::Float(48.0),
            DistortionParam::PreFreq       => PData::Float(70.0),
            DistortionParam::PreBandwidth  => PData::Float(5.0), 
            DistortionParam::PreHighCut    => PData::Float(70.0),
            DistortionParam::Drive         => PData::Float(24.0),
            DistortionParam::Feedback      => PData::Float(1.0), 
            DistortionParam::PostGain      => PData::Float(48.0),
            DistortionParam::PostFreq      => PData::Float(70.0),
            DistortionParam::PostBandwidth => PData::Float(5.0), 
            DistortionParam::PostHighCut   => PData::Float(70.0),
            DistortionParam::OutGain       => PData::Float(24.0),
            DistortionParam::Waveshaper    => PData::Int(6),     
            DistortionParam::ReturnLevel   => PData::Float(1.0), 
        }
    }
    fn value_type(&self) -> ValType {
        match self {
            DistortionParam::PreGain       => ValType::VtFloat,
            DistortionParam::PreFreq       => ValType::VtFloat,
            DistortionParam::PreBandwidth  => ValType::VtFloat,
            DistortionParam::PreHighCut    => ValType::VtFloat,
            DistortionParam::Drive         => ValType::VtFloat,
            DistortionParam::Feedback      => ValType::VtFloat,
            DistortionParam::PostGain      => ValType::VtFloat,
            DistortionParam::PostFreq      => ValType::VtFloat,
            DistortionParam::PostBandwidth => ValType::VtFloat,
            DistortionParam::PostHighCut   => ValType::VtFloat,
            DistortionParam::OutGain       => ValType::VtFloat,
            DistortionParam::Waveshaper    => ValType::VtInt,  
            DistortionParam::ReturnLevel   => ValType::VtFloat,
        }
    }
    fn moverate(&self) -> f32 {
        match self {
            DistortionParam::PreGain       => 1.0,
            DistortionParam::PreFreq       => 1.0,
            DistortionParam::PreBandwidth  => 1.0,
            DistortionParam::PreHighCut    => 1.0,
            DistortionParam::Drive         => 1.0,
            DistortionParam::Feedback      => 1.0,
            DistortionParam::PostGain      => 1.0,
            DistortionParam::PostFreq      => 1.0,
            DistortionParam::PostBandwidth => 1.0,
            DistortionParam::PostHighCut   => 1.0,
            DistortionParam::OutGain       => 1.0,
            DistortionParam::Waveshaper    => 1.0,
            DistortionParam::ReturnLevel   => 1.0,
        }
    }
}

impl DistortionParam {
    #[inline] pub fn new_runtime() -> DistortionParamArrayRT {
        DistortionParamArrayRT::new_with(|x| match x {
            DistortionParam::PreGain       => DistortionParamRT::new(DistortionParam::PreGain      ),
            DistortionParam::PreFreq       => DistortionParamRT::new(DistortionParam::PreFreq      ),
            DistortionParam::PreBandwidth  => DistortionParamRT::new(DistortionParam::PreBandwidth ),
            DistortionParam::PreHighCut    => DistortionParamRT::new(DistortionParam::PreHighCut   ),
            DistortionParam::Drive         => DistortionParamRT::new(DistortionParam::Drive        ),
            DistortionParam::Feedback      => DistortionParamRT::new(DistortionParam::Feedback     ),
            DistortionParam::PostGain      => DistortionParamRT::new(DistortionParam::PostGain     ),
            DistortionParam::PostFreq      => DistortionParamRT::new(DistortionParam::PostFreq     ),
            DistortionParam::PostBandwidth => DistortionParamRT::new(DistortionParam::PostBandwidth),
            DistortionParam::PostHighCut   => DistortionParamRT::new(DistortionParam::PostHighCut  ),
            DistortionParam::OutGain       => DistortionParamRT::new(DistortionParam::OutGain      ),
            DistortionParam::Waveshaper    => DistortionParamRT::new(DistortionParam::Waveshaper   ),
            DistortionParam::ReturnLevel   => DistortionParamRT::new(DistortionParam::ReturnLevel  ),
        })
    }
}
