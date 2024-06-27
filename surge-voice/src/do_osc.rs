crate::ix!();

const YES_FM: bool = true;
const NO_FM:  bool = false;

const OSC0:  usize = 0;
const OSC1:  usize = 1;
const OSC2:  usize = 2;

impl SurgeVoice {

    pub fn process_osc(&mut self,runtime: &mut OscillatorRuntime, idx: usize) 
        -> Result<(),SurgeError> 
    {
        let cfg = match idx {
            0 => {
                match self.fm_mode.on() {
                    true  => runtime.gen_process_cfg::<YES_FM, OSC0>(),
                    false => runtime.gen_process_cfg::<NO_FM,  OSC0>(),
                }
            },
            1 => {
                match self.fm_mode {
                    FmConfiguration::TwoToOneToZero => {
                        runtime.gen_process_cfg::<YES_FM, OSC1>()
                    },
                    _ => {
                        runtime.gen_process_cfg::<NO_FM, OSC1>()
                    },
                }
            },
            2 => {
                runtime.gen_process_cfg::<NO_FM, OSC2>()
            }
            _ => unreachable!(),
        };

        let is_wide            = runtime.is_wide;

        let lag_entry = match idx {
            0 => LagEntry::Osc1,
            1 => LagEntry::Osc2,
            2 => LagEntry::Osc3,
            _ => unreachable!(),
        };

        match self.osc[idx] {
            Some(ref mut x) => x.process_block(cfg)?,
            _ => panic!("where is the oscillator!?"),
        }

        let osc_l = match self.osc[idx] { Some(ref mut x) => x.out_l(), _ => panic!(), };
        let osc_r = match self.osc[idx] { Some(ref mut x) => x.out_r(), _ => panic!(), };

        if self.osc_enable[idx] {

            unsafe {
                match is_wide {
                    true => {
                        self.osclevels[lag_entry].multiply_2_blocks_to(
                            osc_l, 
                            osc_r, 
                            runtime.tblock_l.buf.as_mut_ptr(),
                            runtime.tblock_r.buf.as_mut_ptr(), 
                            BLOCK_SIZE_OS_QUAD);
                    },
                    false => {
                        self.osclevels[lag_entry].multiply_block_to(
                            osc_l, 
                            runtime.tblock_l.buf.as_mut_ptr(), 
                            BLOCK_SIZE_OS_QUAD);
                    }
                };
            }

            if self.route[idx] < 2 {
                accumulate_block(
                    runtime.tblock_l.buf.as_mut_ptr(), 
                    self.output[0].as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD);
            }

            if self.route[idx] > 0 {
                accumulate_block(
                    runtime.tblock_r.buf.as_mut_ptr(), 
                    self.output[1].as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD);
            }
        }

        Ok(())
    }
}
