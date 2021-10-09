ix!();

use crate::{
    PatchMetadata,
    SurgeScene,
    PatchParamArrayRT,
};

#[derive(Debug)]
pub struct SurgePatch< 'sr > {
    pub metadata:              PatchMetadata,
    pub params:                PatchParamArrayRT,

    pub scene:                 Vec<SurgeScene<'sr>>,
    pub fx:                    Vec<SurgeEffect<'sr>>,

    pub maybe_tuning:          Option<TuningData>,
    pub maybe_kbmapping:       Option<MappingData>,
    pub maybe_pitchbend_range: Option<PitchBendRange>,
    pub mpe_enabled:           MpeEnableSwitch,
}
