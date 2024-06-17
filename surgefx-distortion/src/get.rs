crate::ix!();

impl Distortion {

    #[inline] pub fn get_waveshape_idx(&self) -> usize {

        let mut ws: i32 = self.pvali(DistortionParam::Waveshaper);

        if ws >= WaveshapeType::count() as i32 {
            ws = 0;
        }

        ws.try_into().unwrap()
    }
}
