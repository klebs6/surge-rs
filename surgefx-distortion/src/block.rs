ix!();

use crate::{
    Distortion,
    DISTORTION_OS,
    DISTORTION_OS_BITS,
};

impl Distortion {

    #[inline] pub fn do_distortion_block<const N: usize, const T: usize>(
        &mut self, 
        k:            usize, 
        wetblock:     &mut WetBlock2::<T>,
        waveshapeidx: i32, 
        feedback:     f32, 
        data_l:        &mut [f32; N],
        data_r:        &mut [f32; N]) 
    {

        let a: f32 = match (k & 16) != 0 { 
            true => 0.00000001, 
            false => -0.00000001 
        }; // denormal thingy

        let l_in: f32 = data_l[k];
        let r_in: f32 = data_r[k];

        for s in 0..DISTORTION_OS {

            self.left  = l_in + feedback * self.left;
            self.right = r_in + feedback * self.right;

            self.lp1.process_sample_nolag(&mut self.left, &mut self.right);

            self.left  = self.tables.lookup_waveshape(waveshapeidx, self.left);
            self.right = self.tables.lookup_waveshape(waveshapeidx, self.right);

            self.left  += a;
            self.right += a; // denormal

            self.lp2.process_sample_nolag(&mut self.left, &mut self.right);

            unsafe {
                *wetblock.li((s + (k << DISTORTION_OS_BITS)) as isize) = self.left;
                *wetblock.ri((s + (k << DISTORTION_OS_BITS)) as isize) = self.right;
            }
        }
    }
}
