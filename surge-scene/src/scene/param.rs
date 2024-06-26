crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum SceneParam {
    Pitch,
    Octave,
    FmDepth,
    FmSwitch,
    Drift,
    NoiseColour,
    KeytrackRoot,
    OscSolo,
    LevelO1,
    LevelO2,
    LevelO3,
    LevelNoise,
    LevelRing12,
    LevelRing23,
    LevelPfg,
    MuteO1,
    MuteO2,
    MuteO3,
    MuteNoise,
    MuteRing12,
    MuteRing23,
    SoloO1,
    SoloO2,
    SoloO3,
    SoloNoise,
    SoloRing12,
    SoloRing23,
    RouteO1,
    RouteO2,
    RouteO3,
    RouteNoise,
    RouteRing12,
    RouteRing23,
    VcaLevel,
    PitchBendRangeDown,
    PitchBendRangeUp,
    VcaVelsense,
    PolyMode,
    PolyLimit,
    Portamento,
    Volume,
    Pan,
    Width,
    SendLevelA,
    SendLevelB,
    F2CutoffIsOffset,
    F2LinkResonance,
    Feedback,
    FilterBlockConfiguration,
    FilterBalance,
    LowCut,
}

impl_trait_defaults!{
    SceneParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    CheckIfModulateable,
    GetControlStyle,
    GetDefaultValueF01,
    GetExtendedValue,
    GetModulation,
    GetMoverate,
    GetSnap,
    SetModulation,
}

impl CheckIfAbsolute for SceneParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for SceneParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Global } 
}

impl GetDefaultParameterValue for SceneParam {

}

impl GetMaxParameterValue     for SceneParam {

}

impl GetParameterValueType    for SceneParam {

}

impl GetControlType           for SceneParam {

}

impl GetExtendRange           for SceneParam {

}

impl GetMinParameterValue     for SceneParam {

}
