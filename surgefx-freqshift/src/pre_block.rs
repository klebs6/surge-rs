ix!();

use crate::*;

impl FreqShift {

    pub fn pre_block<const N: usize>(
        &mut self, 
        k:         usize, 
        wet_lr:    &mut WetBlock,
        wet_li_ri: &mut WetBlock,
        wet_lr_rr: &mut WetBlock) 
    {
        const FIR_IPOL_N_I32: i32 = FIR_IPOL_N as i32;
        const FIR_IPOL_M_I32: i32 = FIR_IPOL_M as i32;
        const MAX_DELAY:      i32 = FREQSHIFT_MAX_DELAY_LENGTH as i32;

        self.time.process();

        let i_dtime: i32 = 
            maxi(
                FIR_IPOL_N_I32 + (N as i32),  
                mini(
                    self.time.v as i32, 
                    MAX_DELAY - FIR_IPOL_N_I32 - 1
                )
            );

        let rp: i32 = self.wpos - i_dtime + (k as i32);

        let sinc: i32 = FIR_IPOL_N_I32 *
            limit_range(
                (
                    FIR_IPOL_M_I32 * 
                    ((((i_dtime + 1) as f32) - self.time.v) as i32)
                ) as i32, 
                0, 
                FIR_IPOL_M_I32 - 1
            );

        wet_lr.l[k] = 0.0;
        wet_lr.r[k] = 0.0;

        for i in 0..FIR_IPOL_N_I32 
        {
            let idx: usize = ((rp - i) & (MAX_DELAY - 1)).try_into().unwrap();

            let sincidx: usize = (sinc + FIR_IPOL_N_I32 - i).try_into().unwrap();

            wet_lr.l[k] += 
                self.buffer[[0,idx]] * 
                self.tables.sinctable_1x( sincidx );

            wet_lr.r[k] += 
                self.buffer[[1,idx]] * 
                self.tables.sinctable_1x( sincidx );
        }

        // do freqshift (part I)
        self.o1_l.process();
        wet_lr_rr.l[k] = wet_lr.l[k] * (self.o1_l.r as f32);
        wet_li_ri.l[k] = wet_lr.l[k] * (self.o1_l.i as f32);

        self.o1_r.process();
        wet_lr_rr.r[k] = wet_lr.r[k] * (self.o1_r.r as f32);
        wet_li_ri.r[k] = wet_lr.r[k] * (self.o1_r.i as f32);
    }
}
