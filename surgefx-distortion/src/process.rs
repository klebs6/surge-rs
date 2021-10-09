ix!();

use crate::{
    Distortion,
    DistortionParam,
    DISTORTION_OS_BITS,
};

//pub const DISTORTION_OVERSAMPLE_BLOCK_SIZE: usize = BLOCK_SIZE << DISTORTION_OS_BITS;

impl Process for Distortion<'sr> {

    // eventually loosen the constraint on N
    #[allow(clippy::assertions_on_constants)] 
    fn process<const N: usize>( &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        assert!(DISTORTION_OS_BITS == 2);
        assert!(32 == N);

        let drive:     f32 = self.pvalf_extended(DistortionParam::Drive);
        let outgain:   f32 = self.pvalf(DistortionParam::OutGain);

        let mut wetblock = 
            WetBlock2::<128>::default();

        // TODO fix denormals!
        if self.bi == 0 {
            self.update();
        }

        self.bi = (self.bi + 1) & SLOWRATE_M1 as i32;

        unsafe {
            self.band1.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);
        }

        self.drive.set_target_smoothed(self.tables.db_to_linear(drive));
        self.outgain.set_target_smoothed(self.tables.db_to_linear(outgain));

        let feedback:   f32 = self.pvalf(DistortionParam::Feedback);
        let waveshapeidx    = self.get_waveshape_idx();

        unsafe {
            self.drive.multiply_2_blocks(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                BLOCK_SIZE_QUAD);
        }

        for k in 0..N {
            self.do_distortion_block::<N,128>(
                k,
                &mut wetblock, 
                waveshapeidx,
                feedback,
                data_l,
                data_r);
        }

        self.hr_a.process_block_downsample_by_two(
            wetblock.l(), 
            wetblock.r(), 
            Some(128),
            Some(data_l.as_mut_ptr()),
            Some(data_r.as_mut_ptr())
        );

        self.hr_b.process_block_downsample_by_two(
            wetblock.l(), 
            wetblock.r(), 
            Some(64), 
            Some(data_l.as_mut_ptr()), 
            Some(data_r.as_mut_ptr())
        );

        unsafe {
            self.outgain.multiply_2_blocks_to(
                wetblock.l(), 
                wetblock.r(), 
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                block_size_quad![N]
            );

            self.band2.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);
        }
    }
}
