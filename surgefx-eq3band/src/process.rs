ix!();

use crate::{
    Eq3BandParam,
    Eq3Band,
};

impl Process for Eq3Band<'sr> {

    fn process<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        if self.block_increment == 0 {
            self.update();
        }

        self.block_increment = 
            (self.block_increment + 1) & SLOWRATE_M1 as i32;

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

        let gain_db = self.pvalf(Eq3BandParam::Gain);

        self.gain.set_target_smoothed(
            self.tables.db_to_linear(gain_db)
        );

        unsafe {
            self.gain.multiply_2_blocks(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                BLOCK_SIZE_QUAD);
        }

    }
}
