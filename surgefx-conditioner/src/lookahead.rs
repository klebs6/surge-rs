ix!();

use crate::{
    Conditioner,
    CONDITIONER_LOOKAHEAD,
    CONDITIONER_LOOKAHEAD_BITS,
};

impl Conditioner<'sr> {

    pub fn get_lookahead(&self) -> f32 {
        let mut la: f32 = self.lamax[CONDITIONER_LOOKAHEAD - 2];

        la = (2.0 * la).sqrt(); // RMS test
        la = maxf(1.0, la); // * outscale_inv);
        la
    }

    pub fn do_lookahead(&mut self) {

        let mut of: i32 = 0;

        for i in 0..CONDITIONER_LOOKAHEAD_BITS {

            let nextof:    i32   = of + ((CONDITIONER_LOOKAHEAD >> i) as i32);

            let store_idx: usize = (nextof + (self.bufpos >> (i + 1))) as usize;

            let load_idx1: usize = (of + (self.bufpos >> i))           as usize;
            let load_idx2: usize = (of + ((self.bufpos >> i) ^ 0x1))   as usize;

            self.lamax[store_idx] = maxf(self.lamax[load_idx1], self.lamax[load_idx2]);

            of = nextof;
        }
    }
}
