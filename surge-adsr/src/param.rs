crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameter_with_runtime]
pub enum AdsrParam {
    Attack,
    Decay,
    Sustain,
    Release,
    AttackShape,
    DecayShape,
    ReleaseShape,
    Mode,
}

impl GetParameterControlGroup for AdsrParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Env } 
}
