ix!();

use crate::{
    SurgeVoice,
    LagEntry,
    OscillatorRuntime,
};

impl SurgeVoice {

    pub fn process_ring(&mut self,
        runtime: &mut OscillatorRuntime, 
        idx: usize) 
    {
        let is_wide      = runtime.is_wide;

        let (idx1, idx2) = match idx {
            0 => (0,1),
            1 => (1,2),
            _ => unreachable!(),
        };

        let route_idx = match idx {
            0 => 3,
            1 => 4,
            _ => unreachable!(),
        };

        let lag_entry = match idx {
            0 => LagEntry::Ring12,
            1 => LagEntry::Ring23,
            _ => unreachable!(),
        };

        let osc_1l = match self.osc[idx1] { Some(ref mut x) => x.out_l(), _ => panic!(), };
        let osc_1r = match self.osc[idx1] { Some(ref mut x) => x.out_r(), _ => panic!(), };

        let osc_2l = match self.osc[idx2] { Some(ref mut x) => x.out_l(), _ => panic!(), };
        let osc_2r = match self.osc[idx2] { Some(ref mut x) => x.out_r(), _ => panic!(), };

        match is_wide {
            true => {
                mul_block(
                    osc_1l, 
                    osc_2l, 
                    runtime.tblock_l.buf.as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD 
                );

                mul_block(
                    osc_1r, 
                    osc_2r, 
                    runtime.tblock_r.buf.as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD
                );

                unsafe {
                    self.osclevels[lag_entry].multiply_2_blocks(
                        runtime.tblock_l.buf.as_mut_ptr(), 
                        runtime.tblock_r.buf.as_mut_ptr(), 
                        BLOCK_SIZE_OS_QUAD
                    );
                }
            },
            false => {
                mul_block(
                    osc_1l, 
                    osc_2l, 
                    runtime.tblock_l.buf.as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD);

                unsafe {
                    self.osclevels[lag_entry].multiply_block(
                        runtime.tblock_l.buf.as_mut_ptr(), 
                        BLOCK_SIZE_OS_QUAD);
                }
            },
        }

        if self.route[route_idx] < 2 { 
            accumulate_block(
                runtime.tblock_l.buf.as_mut_ptr(), 
                self.output[0].as_mut_ptr(), 
                BLOCK_SIZE_OS_QUAD);  
        }

        if self.route[route_idx] > 0 { 
            if is_wide {
                accumulate_block(
                    runtime.tblock_r.buf.as_mut_ptr(), 
                    self.output[1].as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD);
            } else {
                accumulate_block(
                    runtime.tblock_l.buf.as_mut_ptr(), 
                    self.output[1].as_mut_ptr(), 
                    BLOCK_SIZE_OS_QUAD);
            }
        }
    }
}
