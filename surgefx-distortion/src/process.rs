ix!();

use crate::*;

//pub const DISTORTION_OVERSAMPLE_BLOCK_SIZE: usize = BLOCK_SIZE << DISTORTION_OS_BITS;

impl Process for Distortion {

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

        self.wetblock.clear();

        self.maybe_update();

        self.process_band1(data_l, data_r);

        self.update_drive(drive);
        self.update_outgain(outgain);

        let feedback:   f32 = self.pvalf(DistortionParam::Feedback);
        let waveshapeidx    = self.get_waveshape_idx();

        self.process_drive(data_l, data_r);

        self.process_blocks(
            data_l,
            data_r,
            waveshapeidx,
            feedback
        );

        self.process_halfrate(data_l, data_r);

        self.process_outgain(data_l, data_r);

        self.process_band2(data_l, data_r);
    }
}

impl Distortion {

    #[inline] fn process_band1<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        unsafe {
            self.band1.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);
        }
    }

    #[inline] fn process_drive<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        unsafe {
            self.drive.multiply_2_blocks(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                BLOCK_SIZE_QUAD);
        }
    }

    #[inline] fn process_blocks<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N],
        waveshapeidx: i32, 
        feedback: f32) 
    {
        for k in 0..N {
            self.do_distortion_block::<N,128>(
                k,
                waveshapeidx,
                feedback,
                data_l,
                data_r);
        }
    }

    #[inline] fn process_halfrate<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N])
    {
        self.hr_a.process_block_downsample_by_two(
            self.wetblock.l(), 
            self.wetblock.r(), 
            Some(128),
            Some(data_l.as_mut_ptr()),
            Some(data_r.as_mut_ptr())
        );

        self.hr_b.process_block_downsample_by_two(
            self.wetblock.l(), 
            self.wetblock.r(), 
            Some(64), 
            Some(data_l.as_mut_ptr()), 
            Some(data_r.as_mut_ptr())
        );
    }

    #[inline] fn process_band2<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        unsafe {
            self.band2.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None);
        }
    }

    #[inline] fn process_outgain<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]) 
    {
        unsafe {
            self.outgain.multiply_2_blocks_to(
                self.wetblock.l(), 
                self.wetblock.r(), 
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                block_size_quad![N]
            );
        }
    }
}
