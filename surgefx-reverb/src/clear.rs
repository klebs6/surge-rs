ix!();

use crate::{
    Reverb,
    REVERB_PREDELAY_QUADS,
    REVERB_DELAY_QUADS,
};

impl ClearBuffers for Reverb {

    fn clear_buffers(&mut self) {
        unsafe {
            clear_block::<REVERB_PREDELAY_QUADS>(self.predelay.as_mut_ptr());
            clear_block::<REVERB_DELAY_QUADS>(self.delay.as_mut_ptr());
        }
    }
}

