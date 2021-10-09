
ix!();

use crate::{
    FreqShift,
};

impl FreqShift<'sr> {

    pub fn post_block<const N: usize>(&mut self, 
        k:       usize, 
        data_l:  &mut [f32; N],
        data_r:  &mut [f32; N],
        wet_lr:   &mut WetBlock,
        wet_li_ri: &mut WetBlock,
        wet_lr_rr: &mut WetBlock) 
    {
        const MAX_DELAY: i32 = FREQSHIFT_MAX_DELAY_LENGTH as i32;

        self.o2_l.process();
        wet_lr_rr.l[k] *= self.o2_l.r as f32;
        wet_li_ri.l[k] *= self.o2_l.i as f32;

        self.o2_r.process();
        wet_lr_rr.r[k] *= self.o2_r.r as f32;
        wet_li_ri.r[k] *= self.o2_r.i as f32;

        wet_lr.l[k] = 2.0 * (wet_lr_rr.l[k] + wet_li_ri.l[k]);
        wet_lr.r[k] = 2.0 * (wet_lr_rr.r[k] + wet_li_ri.r[k]);

        let wp: i32 = (self.wpos + (k as i32)) & (MAX_DELAY - 1);

        self.feedback.process();

        self.buffer[[0,wp as usize]] = 
            data_l[k] + 
            (
                self.tables.lookup_waveshape(
                    0, 
                    wet_lr.l[k] * self.feedback.v
                ) as f32
            );

        self.buffer[[1,wp as usize]] = 
            data_r[k] + 
            (
                self.tables.lookup_waveshape(
                    0, 
                    wet_lr.r[k] * self.feedback.v
                ) as f32
            );
    }
}
