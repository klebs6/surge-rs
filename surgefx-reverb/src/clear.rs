crate::ix!();

impl ClearBuffers for Reverb {

    fn clear_buffers(&mut self) -> Result<(),SurgeError> {

        unsafe {
            clear_block::<REVERB_PREDELAY_QUADS>(self.predelay.as_mut_ptr())?;
            clear_block::<REVERB_DELAY_QUADS>(self.delay.as_mut_ptr())?;
        }

        Ok(())
    }
}

