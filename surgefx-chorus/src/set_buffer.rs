ix!();

use crate::{
    Chorus,
};

impl<'sr> Chorus<'sr> {

    pub fn chorus_set_buffer<const N: usize>(&mut self, tbuffer: &mut TBuffer) {

        if self.wpos + (N as i32) >= (CHORUS_MAX_DELAY_LENGTH as i32) {

            for k in 0..N {
                self.buffer[
                    ((self.wpos as usize + k) & (CHORUS_MAX_DELAY_LENGTH - 1)) as usize
                ] = tbuffer.fb[k];
            }

        } else {

            unsafe {
                copy_block(tbuffer.fb(), 
                    self.buffer.as_mut_ptr().offset(self.wpos as isize), 
                    block_size_quad![N]);
            }
        }

        if self.wpos == 0 {
            for k in 0..FIR_IPOL_N {
                // copy buffer so FIR-core doesn't have to wrap
                self.buffer[k + CHORUS_MAX_DELAY_LENGTH] = self.buffer[k]; 
            }
        }
    }
}
