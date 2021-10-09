ix!();

use crate::{
    Chorus,
};

impl<'sr> Process for Chorus<'sr> {

    fn process<const N: usize>(&mut self, data_l: &mut [f32; N], data_r: &mut [f32; N]) {

        self.update();

        let mut tbuffer = TBuffer::new(N);
        tbuffer.clear_blocks();

        for k in 0..N {
            self.do_chorus_block::<N>(k, tbuffer.l(), tbuffer.r());
        }

        unsafe {
            self.lp.process_block_stereo(
                tbuffer.l(),
                tbuffer.r(),
                None);

            self.hp.process_block_stereo(
                tbuffer.l(),
                tbuffer.r(),
                None);
        }

        add_block(
            tbuffer.l(), 
            tbuffer.r(), 
            tbuffer.fb(), 
            block_size_quad![N]);

        unsafe {
            self.feedback.multiply_block(tbuffer.fb(), block_size_quad![N]);

            hardclip_block(
                tbuffer.fb(), 
                block_size_quad![N]);
        }

        accumulate_block(
            data_l.as_mut_ptr(), 
            tbuffer.fb(), 
            block_size_quad![N]);

        accumulate_block(
            data_r.as_mut_ptr(), 
            tbuffer.fb(), 
            block_size_quad![N]);

        self.chorus_set_buffer::<N>(&mut tbuffer);

        // scale width
        let mut mid_side = MSBlock::new(N);

        unsafe {
            encode_mid_side(
                tbuffer.l(), 
                tbuffer.r(), 
                mid_side.m(), 
                mid_side.s(), 
                block_size_quad![N]);

            self.width.multiply_block(
                mid_side.s(), 
                block_size_quad![N]);

            decode_mid_side(
                mid_side.m(), 
                mid_side.s(), 
                tbuffer.l(), 
                tbuffer.r(), 
                block_size_quad![N]);

            //for fear drives one for greater
            //pleasure to seek some enlightment in
            //a different world, in a different
            //experience -- a transcendental
            //something-or-other but, without
            //changing our life in every
            //relationship without knowing what
            //love means -- not an idea, but
            //actually love to love...  one must
            //always live in the past, and you
            //cannot escape it and one can only
            //understand it look at it learn about
            //it be extraordinarily aware of it
            //without any pretension without any
            //hypocrisy do you ever ask yourself,
            //is it at all possible to help
            //another?  J Krishnamurti, Amsterdam
            //1969
            self.mix.fade_2_blocks_to(
                data_l.as_mut_ptr(), 
                tbuffer.l(), 
                data_r.as_mut_ptr(), 
                tbuffer.r(), 
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                block_size_quad![N]);
        }

        self.wpos += N as i32;
        self.wpos &= (CHORUS_MAX_DELAY_LENGTH - 1) as i32;
    }
}

