ix!();

use crate::{Scale,KeyboardMapping};

pub trait Note2Pitch {

    fn n2p<T: MyFloat, const IGNORE_TUNING: bool>(&self,x: T) -> T;
    fn n2pinv<T: MyFloat,const IGNORE_TUNING: bool>(&self,x: T) -> T;

    fn n2p_tuningctr<   T: MyFloat>(&self,x: T) -> T;
    fn n2pinv_tuningctr<T: MyFloat>(&self,x: T) -> T;

    fn note_to_omega<T: MyFloat, const IGNORE_TUNING: bool>(&self,x: T) -> (T, T);
    fn pitch2omega<T: MyFloat>(&self, x: T) -> T;
}

pub trait Retune: Init {
    fn retune_to_scale(&mut self, s: & Scale) -> bool ;
    fn retune_to_standard_tuning(&mut self) -> bool { 
        self.init(); 
        true
    }
}

pub trait KeyboardRemapper {
    fn remap_to_keyboard(&mut self,k: &KeyboardMapping) -> bool ;
    fn remap_to_standard_keyboard(&mut self) -> bool ; 
}

pub trait ScaleNote {
    fn scale_constant_note(&self) -> i32;
    fn scale_constant_pitch(&self) -> f32; 

    /// Obviously this is the inverse of the above
    fn scale_constant_pitch_inv(&self) -> f32 {
        1.0 / self.scale_constant_pitch() 
    }
}
