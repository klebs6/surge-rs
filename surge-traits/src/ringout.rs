crate::ix!();

#[enum_dispatch]
pub trait GetRingout {
    fn get_ringout(&self)         -> Ringout;
    fn get_ringout_counter(&self) -> NumberOfBlocks;
}

#[enum_dispatch]
pub trait SetRingout {
    fn ringout_counter_incr(&mut self);
    fn ringout_counter_reset(&mut self);
}

#[enum_dispatch]
pub trait ProcessRingout : 
StereoProcess 
+ ProcessOnlyControl 
+ GetRingout 
+ SetRingout 
{
    /// # Safety
    ///
    /// must be able to access N valid contiguous items 
    /// from data_l and data_r
    unsafe fn process_ringout<const N: usize>(
        &mut self, 
        data_l: *mut f32, 
        data_r: *mut f32, 
        indata_present: bool

    ) -> Result<OutputDataPresent,AlignmentError>
    {
        let data_l: &mut [f32; N] = std::slice::from_raw_parts_mut(data_l, N).try_into().unwrap();
        let data_r: &mut [f32; N] = std::slice::from_raw_parts_mut(data_r, N).try_into().unwrap();

        let mut do_process = false;
        let mut decay_max = 0;

        match self.get_ringout() {
            Ringout::On {counter: _, decay } => {
                match indata_present {
                    true  => {
                        self.ringout_counter_reset();
                        do_process = true;
                        decay_max = decay;
                    },
                    false => {
                        self.ringout_counter_incr();
                    },
                }
            },
            Ringout::Off => {
                do_process = true;
            },
        }

        if (self.get_ringout_counter() < decay_max) || do_process {
            self.stereo_process::<N>(data_l,data_r)?;
            Ok(true)
        } else {
            self.process_only_control::<N>();
            Ok(false)
        }
    }
}
