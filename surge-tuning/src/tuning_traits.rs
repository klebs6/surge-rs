crate::ix!();

#[enum_dispatch]
pub trait Note2Pitch {

    fn n2p<T: MyFloat, const IGNORE_TUNING: bool>(&self,x: T) -> T;
    fn n2pinv<T: MyFloat,const IGNORE_TUNING: bool>(&self,x: T) -> T;

    fn n2p_tuningctr<   T: MyFloat>(&self,x: T) -> T;
    fn n2pinv_tuningctr<T: MyFloat>(&self,x: T) -> T;

    fn note_to_omega<T: MyFloat, const IGNORE_TUNING: bool>(&self,x: T) -> (T, T);
    fn pitch2omega<T: MyFloat>(&self, x: T) -> T;
}

/*
#[enum_dispatch]
pub trait Retune: Initialize + RetuneToScale  {
    fn retune_to_standard_tuning(&mut self) { 
        self.init(); 
    }
}
*/

#[enum_dispatch]
pub trait CurrentScale {
    fn current_scale(&self) -> Scale;
}

#[enum_dispatch]
pub trait CurrentTuning {
    fn current_tuning(&self) -> SurgeTuning;
}

#[enum_dispatch]
pub trait KeyboardRemapper {
    fn remap_to_keyboard(&mut self,k: &KeyboardMapping) -> bool ;
    fn remap_to_standard_keyboard(&mut self) -> bool ; 
}

#[enum_dispatch]
pub trait ScaleNote {
    fn scale_constant_note(&self) -> i32;
    fn scale_constant_pitch(&self) -> f32; 

    /// Obviously this is the inverse of the above
    fn scale_constant_pitch_inv(&self) -> f32 {
        1.0 / self.scale_constant_pitch() 
    }
}

#[enum_dispatch]
pub trait CurrentScaleCount {

    fn current_scale_count<T>(&self) -> T 
    where 
        T: TryFrom<i8>,
        <T as std::convert::TryFrom<i8>>::Error: std::fmt::Debug ;

}

#[enum_dispatch]
pub trait CurrentTuningIsStandard {
    fn current_tuning_is_standard(&self) -> bool;
}

#[enum_dispatch]
pub trait CurrentMappingIsStandard {
    fn current_mapping_is_standard(&self) -> bool;
}

#[enum_dispatch]
pub trait CurrentScaleRawContents {
    fn current_scale_raw_contents(&self) -> TuningData;
}

#[enum_dispatch]
pub trait CurrentMappingRawContents {
    fn current_mapping_raw_contents(&self) -> MappingData;
}

#[enum_dispatch]
pub trait RetuneToScale {
    fn retune_to_scale(&mut self, scale: &Scale) -> bool;
}

#[enum_dispatch]
pub trait RetuneToStandardTuning {
    fn retune_to_standard_tuning(&mut self) -> Result<(),SurgeError>;
}

#[enum_dispatch]
pub trait RemapKeyboard {
    fn remap_to_keyboard(&mut self, kb: &KeyboardMapping) -> Result<bool,SurgeError>;
}

#[enum_dispatch]
pub trait RemapToStandardKeyboard {
    fn remap_to_standard_keyboard(&mut self) -> Result<bool,SurgeError>;
}

#[enum_dispatch]
pub trait GetTablePitch {

    fn get_tablepitch<IDX>(&self, idx: IDX) -> f64 
    where 
        IDX: TryInto<usize>,
        <IDX as std::convert::TryInto<usize>>::Error: std::fmt::Debug;
}

