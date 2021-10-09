ix!();

use crate::{
    Distortion,
    DistortionParam,
};

impl Distortion<'sr> {

    #[inline] pub fn get_waveshape_idx(&self) -> i32 {

        let mut ws: usize = 
            (self.pvali(DistortionParam::Waveshaper) as isize).try_into().unwrap();

        if ws >= WaveshapeType::count() {
            ws = 0;
        }

        ws as i32
    }
}
