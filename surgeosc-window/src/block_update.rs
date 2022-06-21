crate::ix!();

impl WindowOscillator {

    #[inline] pub fn block_update_mono_out(
        &mut self, 
        block_idx: usize, 
        i_win:     &WetBlock1::<4>, 
        i_wave:    &WetBlock1::<4>) 
    {
        self.out.l[block_idx] += {

            let b0 = i_win.buf[0];
            let b1 = i_wave.buf[0];

            let b2 = b0 * b1;

            ((b2 as i32) >> 6) as f32
        };
    }

    #[inline] pub fn block_update_stereo_out(
        &mut self, 
        so:        usize, 
        block_idx: usize, 
        i_win:     &WetBlock1::<4>, 
        i_wave:    &WetBlock1::<4>) {

        let out: i32 = {

            let b0 = i_win.buf[0];
            let b1 = i_wave.buf[0];

            let b2 = b0 * b1;

            ((b2 as i32) >> 7) as i32
        };

        self.out.l[block_idx] += {

            let g = self.gain[[so,0]] as i32;

            ((out * g) >> 6) as f32
        };

        self.out.r[block_idx] += {

            let g = self.gain[[so,1]] as i32;

            ((out * g) >> 6) as f32
        };
    }
}
