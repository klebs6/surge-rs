crate::ix!();

pub trait GetControlType {

    fn control_type(&self)
        -> ControlType { ControlType::Nil }
}

impl<P: ParameterInterface + ?Sized> GetControlType for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn control_type(&self) -> ControlType;
        }
    }
}

enhanced_enum![
    ControlType {
        Nil,
        Percent,
        PercentBidirectional,
        PitchOctave,
        PitchSemi7BP,
        PitchSemi7BPAbsolutable,
        Pitch,
        FMRatio,
        FMRatioInt,
        PBDepth,
        SyncPitch,
        Amplitude,
        ReverbShape,
        Decibel,
        DecibelNarrow,
        DecibelNarrowExtendable,
        DecibelExtraNarrow,
        DecibelAttenuation,
        DecibelAttenuationLarge,
        DecibelFMDepth,
        DecibelExtendable,
        FreqAudible,
        FreqMod,
        FreqHpf,
        FreqShift,
        FreqVocoderLow,
        FreqVocoderHigh,
        Bandwidth,
        EnvTime,
        EnvTimeLfoDecay,
        EnvShape,
        EnvelopeMode,
        DelayModTime,
        ReverbTime,
        ReverbPreDelayTime,
        PortaTime,
        LfoRate,
        LfoShape,
        LfoTrigMode,
        Detuning,
        OscType,
        FxType,
        FxBypass,
        FbConfig,
        FmConfig,
        FilterType,
        FilterSubType,
        WaveshapeType,
        Wt2Window,
        OscCount,
        OscCountWT,
        OscSpread,
        SceneMode,
        SceneSel,
        PolyMode,
        PolyLimit,
        MidiKey,
        MidiKeyOrChannel,
        Bool,
        BoolRelativeSwitch,
        BoolLinkSwitch,
        BoolKeytrack,
        BoolRetrigger,
        BoolUnipolar,
        BoolMute,
        BoolSolo,
        OscRoute,
        StereoWidth,
        BoolFM,
        Character,
        SineOscMode,
        SineFMLegacy,
        CountedSetPercent,// what % through a counted set are you
        VocoderBandcount,
        DistortionWaveshape,
        FlangerPitch,
        FlangerMode,
        FlangerVoices,
        //TODO: unknown is temporary until 
        //we find out the true value
        Unknown, 
    }
];
