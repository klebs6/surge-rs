ix!();

//TODO: this is a little fishy
bitflags! {
    pub struct ControlStyle: u32 {
        const OFF        = 0b00000000000000000000000000000000;
        const HORIZONTAL = 0b00000000000000000000000000000001;
        const VERTICAL   = 0b00000000000000000000000000000010;
        const BIPOLAR    = 0b00000000000000001000000000000000;
        const WHITE      = 0b00000000000000010000000000000000;
        const SEMITONE   = 0b00000000000000100000000000000000;
        const MINI       = 0b00000000000001000000000000000000;
        const META       = 0b00000000000010000000000000000000;
        const EASY       = 0b00000000000100000000000000000000;
        const HIDE       = 0b00000000001000000000000000000000;
        const NOPOPUP    = 0b00000000010000000000000000000000;
    }
}

enhanced_enum![
    ControlGroup {
        Nil,
        Global,
        Osc,
        Mix,
        Filter,
        Env,
        Lfo,
        Fx,
    }
];

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
