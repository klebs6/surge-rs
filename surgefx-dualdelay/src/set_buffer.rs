ix!();

use crate::{
    DualDelay,
    DUAL_DELAY_MAX_DELAY_LENGTH,
};

impl DualDelay<'sr> {

    pub fn dualdelay_set_buffer(&mut self) {

        if self.wpos + (BLOCK_SIZE as i32) >= (DUAL_DELAY_MAX_DELAY_LENGTH as i32) {

            for k in 0..BLOCK_SIZE
            {
                unsafe {
                    self.buffer[[0, ((self.wpos + k as i32) & (DUAL_DELAY_MAX_DELAY_LENGTH as i32 - 1)) as usize]] = 
                        *self.wetblock.li(k as isize);

                    self.buffer[[1, ((self.wpos + k as i32) & (DUAL_DELAY_MAX_DELAY_LENGTH as i32 - 1)) as usize]] = 
                        *self.wetblock.ri(k as isize);
                }
            }

        } else {

            copy_block(self.wetblock.l(), &mut self.buffer[[0, self.wpos as usize]], BLOCK_SIZE_QUAD);
            copy_block(self.wetblock.r(), &mut self.buffer[[1, self.wpos as usize]], BLOCK_SIZE_QUAD);
        }

        if self.wpos == 0 {

            for k in 0..FIR_IPOL_N
            {
                self.buffer[[0, k + DUAL_DELAY_MAX_DELAY_LENGTH]] =
                    self.buffer[[0, k]]; // copy buffer so FIR-core doesn't have to wrap

                self.buffer[[1, k + DUAL_DELAY_MAX_DELAY_LENGTH]] = self.buffer[[1,k]];
            }
        }
    }
}
