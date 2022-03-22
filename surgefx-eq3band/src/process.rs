ix!();

use crate::*;

impl Process for Eq3Band {

    fn process<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        self.maybe_update();

        self.process_bands(data_l, data_r);

        self.update_gain();

        self.apply_gain(data_l, data_r);
    }
}

impl Eq3Band {

    #[inline] fn process_bands<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        unsafe {
            self.band1.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);

            self.band2.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);

            self.band3.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);
        }
    }

    #[inline] fn apply_gain<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        unsafe {
            self.gain.multiply_2_blocks(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                BLOCK_SIZE_QUAD);
        }
    }
}
