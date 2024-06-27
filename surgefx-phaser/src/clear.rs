crate::ix!();

impl Phaser {

    pub fn suspend_all_biquads(&mut self) -> Result<(),SurgeError> {

        for i in 0_usize..(PHASER_N_BQ_UNITS as usize) {
            self.biquad[i].suspend()?;
        }

        Ok(())
    }

    pub fn clear_blocks(&mut self) -> Result<(),SurgeError> {

        unsafe {
            clear_block::<BLOCK_SIZE_QUAD>(self.l.as_mut_ptr())?;
            clear_block::<BLOCK_SIZE_QUAD>(self.r.as_mut_ptr())?;
        }

        Ok(())
    }
}
