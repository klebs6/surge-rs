ix!();

use crate::{
    FreqShift,
};

impl Process for FreqShift<'sr> {

    fn process<const N: usize>(&mut self, 
        data_l: &mut [f32; N], data_r: &mut [f32; N]) 
    {

        self.update();

        if ! self.inithadtempo && 
            self.time_unit.temposyncratio_inv() != 0.0 
        {
            self.inithadtempo = true;
        } 
           
        let mut wet_lr    = WetBlock::new(BLOCK_SIZE);
        let mut wet_li_ri = WetBlock::new(BLOCK_SIZE);
        let mut wet_lr_rr = WetBlock::new(BLOCK_SIZE);

        for k in 0..BLOCK_SIZE {
            self.pre_block::<N>(
                k, 
                &mut wet_lr, 
                &mut wet_li_ri, 
                &mut wet_lr_rr);
        }

        self.fr.process_block(
            wet_lr_rr.l.as_mut_ptr(), 
            wet_lr_rr.r.as_mut_ptr(), 
            Some(BLOCK_SIZE));

        self.fi.process_block(
            wet_li_ri.l.as_mut_ptr(), 
            wet_li_ri.r.as_mut_ptr(), 
            Some(BLOCK_SIZE));

        self.maybe_do_commented_c_process::<false>();

        for k in 0..BLOCK_SIZE {
            self.post_block(k, 
                data_l, 
                data_r, 
                &mut wet_lr, 
                &mut wet_li_ri, 
                &mut wet_lr_rr);
        }

        unsafe {
            self.mix.fade_2_blocks_to(
                data_l.as_mut_ptr(), 
                wet_lr.l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                wet_lr.r.as_mut_ptr(), 
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                BLOCK_SIZE_QUAD);
        }

        self.wpos += BLOCK_SIZE as i32;
        self.wpos &= (FREQSHIFT_MAX_DELAY_LENGTH as i32) - 1;
    }
}
