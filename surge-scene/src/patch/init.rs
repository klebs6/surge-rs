crate::ix!();

impl SurgePatch {

    pub fn new<'a>(ctor: SceneConstructorHandles<'a>) -> Self {

        let mut x = Self {
            metadata:              PatchMetadata::default(),
            scene:                 vec![SurgeScene::new(ctor)],
            fx:                    Vec::with_capacity(8),
            params:                PatchParam::new_runtime(),
            maybe_tuning:          None,
            maybe_kbmapping:       None,
            maybe_pitchbend_range: None,
            mpe_enabled:           MpeEnableSwitch(false),
        };

        x.params[PatchParam::PolyLimit].val = PData::Int(8);

        x
    }
}

impl SurgePatch {

    pub fn get_param_by_idx(&self, _idx: usize) -> ParamRT<dyn ParameterInterface> {
        todo!();
    }
}

impl Initialize for SurgePatch {
    fn init(&mut self) {
        todo!();
    }
}

impl SaveInto for SurgePatch {
    fn save_into(&mut self, _bytes: &mut Vec<u8>) -> PatchDataSize {
        todo!();
    }
}

impl crate::PatchLoad for SurgePatch {
    fn load_patch(&mut self, 
        _data: c_void, 
        _size: i32, 
        _preset: bool) 
    {
        todo!();
    }
}
