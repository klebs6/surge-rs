crate::ix!();

impl Distortion {

    #[inline] pub fn get_waveshape_idx(&self) -> i32 {

        let mut ws: i32 = self.pvali(DistortionParam::Waveshaper);

        if ws >= WaveshapeType::count() as i32 {
            ws = 0;
        }

        ws
    }
}
