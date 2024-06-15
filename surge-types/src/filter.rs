crate::ix!();

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
