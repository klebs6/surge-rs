crate::ix!();

impl Distortion {

    #[inline] fn denormalizer(k: usize) -> f32 {
        match (k & 16) != 0 { 
            true  => 0.00000001, 
            false => -0.00000001 
        }
    }

    #[inline] fn set_wetblock(
        &mut self,
        idx:          isize) 
    {
        unsafe {
            *self.wetblock.li(idx) = self.left;
            *self.wetblock.ri(idx) = self.right;
        }
    }

    #[inline] fn do_denormalize_channels(&mut self, a: f32) {
        self.left  += a;
        self.right += a; // denormal
    }

    #[inline] fn do_waveshape_channels(&mut self, waveshapeidx: usize) {
        self.left  = self.tables.lookup_waveshape(waveshapeidx, self.left);
        self.right = self.tables.lookup_waveshape(waveshapeidx, self.right);
    }

    #[inline] fn do_feedback_channels(&mut self, l_in: f32, r_in: f32, feedback: f32) {
        self.left  = l_in + feedback * self.left;
        self.right = r_in + feedback * self.right;
    }

    #[inline] pub fn do_distortion_block<const N: usize, const T: usize>(
        &mut self, 
        k:            usize, 
        waveshapeidx: usize, 
        feedback:     f32, 
        data_l:        &mut [f32; N],
        data_r:        &mut [f32; N]) 
    {
        let a: f32 = Self::denormalizer(k);

        let l_in: f32 = data_l[k];
        let r_in: f32 = data_r[k];

        for s in 0..DISTORTION_OS {

            self.do_feedback_channels(l_in,r_in,feedback);

            self.lp1.process_sample_nolag(&mut self.left, &mut self.right);

            self.do_waveshape_channels(waveshapeidx);

            self.do_denormalize_channels(a);

            self.lp2.process_sample_nolag(&mut self.left, &mut self.right);

            let idx = s + (k << DISTORTION_OS_BITS);

            self.set_wetblock(idx as isize);
        }
    }
}
