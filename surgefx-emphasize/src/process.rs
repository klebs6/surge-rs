crate::ix!();

impl StereoProcess for Emphasize {

    fn stereo_process<const N: usize>(
        &mut self, 
        _data_l: &mut [f32; N], 
        _data_r: &mut [f32; N]

    ) -> Result<(),AlignmentError> {

        //todo!();

        Ok(())
    }
}
