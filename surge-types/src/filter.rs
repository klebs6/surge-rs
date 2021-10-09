ix!();

///------------------------------------------------------------------------------------------
///this macro lets us access 
///the coeffmaker indices via the quadfilterunit
///by name.
///
///When specifying the enum, order matters, because it 
///corresponds to the usize value used as index upon deref
///
#[macro_export] macro_rules! coeffidx {
    ($n:ident ; $($idx:ident),* ) => {
        enhanced_enum![
            $n {
                $($idx),*
            }
        ];

        impl<T, const N: usize> std::ops::Index<$n> for [T; N] {
            type Output = T;

            fn index(&self, idx: $n) -> &T {
                &self[idx as usize]
            }
        }
        impl<T, const N: usize> std::ops::IndexMut<$n> for [T; N] {
            fn index_mut(&mut self, idx: $n) -> &mut T {
                &mut self[idx as usize]
            }
        }
    }
}

enhanced_enum![
    FilterTypeIIR {
        LowPass,
        BandPass,
        HighPass,
    }
];

enhanced_enum![
    PoleType {
        FourPole,
        TwoPole,
    }
];


enhanced_enum![
    FilterSubType {
        SVF,
        Rough,
        Smooth,
        Medium,
    }
];

#[allow(non_camel_case_types)]
enhanced_enum![
    FilterType {
        Off,
        Lowpass12dB,
        Lowpass24dB,
        MoogLadder, //lp moog ladder
        Highpass12dB,
        Highpass24dB,
        Bandpass12dB,
        Bandpass24dB,
        Notch,
        Comb,
        SampleAndHold,
        DiodeLadder,
        K35Highpass,
        K35Lowpass,
        Obxd2Pole,
        Obxd4Pole,
        VintageLadderRungeKutta,
        VintageLadderHuovilainen,
        NonlinearFeedback,
        NonlinearStates,
    }
];

enhanced_enum![
    WindowType {
        Triangular,
        Cosine,
        Blend1,
        Blend2,
        Blend3,
        Ramp,
        SineCycle,
        SquareCycle,
        Rectangular,
    }
];

enhanced_enum![
    FilterBlockConfiguration {
        Serial1,
        Serial2,
        Serial3,
        Dual1,
        Dual2,
        Stereo,
        Ring,
        Wide
    }
];

impl FilterBlockConfiguration {
    pub fn is_dual(&self) -> bool {
        matches![self,
        FilterBlockConfiguration::Dual1
            | FilterBlockConfiguration::Dual2]
    }
    pub fn is_serial(&self) -> bool {
        matches![self,
        FilterBlockConfiguration::Serial1
            | FilterBlockConfiguration::Serial2 
            | FilterBlockConfiguration::Serial3]
    }
    pub fn is_stereo(&self) -> bool {
        matches![self, FilterBlockConfiguration::Stereo]
    }

    pub fn is_wide(&self) -> bool {
        matches![self, FilterBlockConfiguration::Wide]
    }
}

#[derive(Debug)]
pub struct FilterUnit {
    pub params: FilterParamArray::<FilterParamRT>,
}

impl Default for FilterUnit {
    fn default() -> Self {
        todo!();
    }
}

enhanced_enum![
    FilterParam {
        Type,
        SubType,
        Cutoff,
        Resonance,
        EnvelopeMode,
        KeyTrack,
    }
];

rt![FilterParam];

impl Param for FilterParam {

    //TODO: check these defaults, perhaps there are better ones
    //what type is KeyTrack?
    //how do we select one of the cases in an enum switch?
    fn control_group(&self) -> ControlGroup { ControlGroup::Filter } 

    fn control_type(&self) -> ControlType {
        match self {
            FilterParam::Type         => ControlType::FilterType,
            FilterParam::SubType      => ControlType::FilterSubType,
            FilterParam::Cutoff       => ControlType::FreqAudible,
            FilterParam::Resonance    => ControlType::Unknown,
            FilterParam::EnvelopeMode => ControlType::EnvelopeMode,
            FilterParam::KeyTrack     => ControlType::Unknown,
        }
    }
    fn default_value(&self) -> PData {
        match self {
            FilterParam::Type         => PData::Int(0),
            FilterParam::SubType      => PData::Int(0),
            FilterParam::Cutoff       => PData::Float(0.5),
            FilterParam::Resonance    => PData::Float(0.5),
            FilterParam::EnvelopeMode => PData::Float(0.5),
            FilterParam::KeyTrack     => PData::Float(0.5),
        }
    }
    fn min_value(&self) -> PData {
        match self {
            FilterParam::Type         => PData::Int(0),
            FilterParam::SubType      => PData::Int(0),
            FilterParam::Cutoff       => PData::Float(0.0),
            FilterParam::Resonance    => PData::Float(0.0),
            FilterParam::EnvelopeMode => PData::Float(0.0),
            FilterParam::KeyTrack     => PData::Float(0.0),
        }
    }
    fn max_value(&self) -> PData {
        match self {
            FilterParam::Type         => PData::Int(1),
            FilterParam::SubType      => PData::Int(1),
            FilterParam::Cutoff       => PData::Float(1.0),
            FilterParam::Resonance    => PData::Float(1.0),
            FilterParam::EnvelopeMode => PData::Float(1.0),
            FilterParam::KeyTrack     => PData::Float(1.0),
        }
    }
    fn modulateable(&self) -> bool {
        match self {
            FilterParam::Type         => false,
            FilterParam::SubType      => false,
            FilterParam::Cutoff       => true,
            FilterParam::Resonance    => true,
            FilterParam::EnvelopeMode => false,
            FilterParam::KeyTrack     => false,
        }
    }
    fn value_type(&self) -> ValType {
        match self {
            FilterParam::Type         => ValType::VtFloat,
            FilterParam::SubType      => ValType::VtFloat,
            FilterParam::Cutoff       => ValType::VtFloat,
            FilterParam::Resonance    => ValType::VtFloat,
            FilterParam::EnvelopeMode => ValType::VtFloat,
            FilterParam::KeyTrack     => ValType::VtFloat,
        }
    }
    fn moverate(&self) -> f32 {
        match self {
            FilterParam::Type         => 1.0,
            FilterParam::SubType      => 1.0,
            FilterParam::Cutoff       => 1.0,
            FilterParam::Resonance    => 1.0,
            FilterParam::EnvelopeMode => 1.0,
            FilterParam::KeyTrack     => 1.0,
        }
    }
}
