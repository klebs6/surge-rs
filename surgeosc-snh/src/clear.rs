crate::ix!();

impl SampleAndHoldOscillator {

    pub fn clear_blocks(&mut self, stereo: bool) -> Result<(),SurgeError> {

        unsafe {
            clear_block::<BLOCK_SIZE_OS_QUAD>(
                &mut self.blitter.oscbuffer_l[self.blitter.bufpos as usize]
            )?;

            if stereo {
                clear_block::<BLOCK_SIZE_OS_QUAD>(
                    &mut self.blitter.oscbuffer_r[self.blitter.bufpos as usize]
                )?;
            }

            clear_block::<BLOCK_SIZE_OS_QUAD>(
                &mut self.blitter.dcbuffer[self.blitter.bufpos as usize]
            )?;
        }

        Ok(())
    }
}
