crate::ix!();

enhanced_enum![
    PatchParam {
        SceneActive,
        SceneMode,
        SceneMorph,
        SplitKey,
        Volume,
        PolyLimit,
        FxBypass,
        FxDisable,
        Character,
    }
];

rt![PatchParam];

impl ParameterInterface for PatchParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Global } 
}

impl PatchParam {
    pub fn new_runtime() -> PatchParamArrayRT {
        PatchParamArrayRT::new_with(|x| match x {
            PatchParam::SceneActive      => PatchParamRT::new(PatchParam::SceneActive),
            PatchParam::SceneMode        => PatchParamRT::new(PatchParam::SceneMode),
            PatchParam::SceneMorph       => PatchParamRT::new(PatchParam::SceneMorph),
            PatchParam::SplitKey         => PatchParamRT::new(PatchParam::SplitKey),
            PatchParam::Volume           => PatchParamRT::new(PatchParam::Volume),
            PatchParam::PolyLimit        => PatchParamRT::new(PatchParam::PolyLimit),
            PatchParam::FxBypass         => PatchParamRT::new(PatchParam::FxBypass),
            PatchParam::FxDisable        => PatchParamRT::new(PatchParam::FxDisable),
            PatchParam::Character        => PatchParamRT::new(PatchParam::Character),
        })
    }
}
