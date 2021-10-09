ix!();

use crate::{
    RotarySpeaker,
};

impl RotarySpeaker<'sr> {

    pub fn do_rotaryspeaker_pre_block<const N: usize>(&mut self, 
        k:        usize,
        wetblock: &mut WetBlockULS::<BLOCK_SIZE>,
        data_l:   &mut [f32; N],
        data_r:   &mut [f32; N]
    ) {
        let input: f32 = 0.5 * ( data_l[k] + data_r[k] );

        wetblock.u[k] = input;
        wetblock.l[k] = input;
        self.drive.process();
    }
}
