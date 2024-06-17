crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum PatchParam {
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

impl GetControlType for PatchParam {

}

impl GetDefaultParameterValue for PatchParam {

}

impl GetExtendRange for PatchParam {

}

impl GetMaxParameterValue for PatchParam {

}

impl GetMinParameterValue for PatchParam {

}

impl GetParameterValueType for PatchParam {

}

impl GetControlGroup for PatchParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Global } 
}

impl GetDefaultValueF01 for PatchParam {

}

impl GetExtendedValue for PatchParam {

}

impl SetModulation for PatchParam {

}

impl GetModulation for PatchParam {

}

impl CheckIfCanSnap for PatchParam {

}

impl CheckIfAbsolute for PatchParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl CheckIfCanBeAbsolute for PatchParam {

}

impl CheckIfCanExtendRange for PatchParam {

}

impl CheckIfCanTemposync for PatchParam {

}

impl CheckIfAffectsOtherParameters for PatchParam {

}

impl GetSnap for PatchParam {

}

impl GetMoverate for PatchParam {

}

impl CheckIfModulateable for PatchParam {

}

impl GetControlStyle for PatchParam {

}
