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

impl GetControlGroup for PatchParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Global } 
}
