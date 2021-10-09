ix!();

use crate::{
    Reverb,
    ReverbPreset,
    ReverbParam,
};

impl Process for Reverb<'sr> {

    fn process<const N: usize>(&mut self, data_l: &mut [f32; N], data_r: &mut [f32; N]) {

        let f_roomsize   = self.pvalf(ReverbParam::RoomSize);
        let f_decaytime  = self.pvalf(ReverbParam::DecayTime);
        let f_mix        = self.pvalf(ReverbParam::Mix);
        let f_width      = self.pvalf(ReverbParam::Width);
        let i_shape      = self.pvali(ReverbParam::RoomShape);

        let roomsize_diff  = (f_roomsize  - self.lastf[ReverbParam::RoomSize as usize]).abs();
        let decaytime_diff = (f_decaytime - self.lastf[ReverbParam::DecayTime as usize]).abs();

        let preset_update_requested: bool = i_shape != (self.preset as i32);
        let preset_need_update:      bool = ( self.b == 0) && (roomsize_diff > 0.001);

        if preset_need_update || preset_update_requested {
            self.load_preset(ReverbPreset::try_from(i_shape as usize).unwrap());
        }

        if decaytime_diff > 0.001 {
            self.update_rtime();
        }

        self.maybe_recalc_coefficients();

        self.mix.set_target_smoothed(f_mix);
        self.width.set_target_smoothed(self.tables.db_to_linear(f_width));

        let mut wetblock = WetBlock::new(BLOCK_SIZE);

        self.do_reverb(&mut wetblock, data_l, data_r);

        self.process_filter_bands(&mut wetblock);

        let mut ms = MSBlock::new(BLOCK_SIZE); // scale width

        let wet_l  = wetblock.l.as_mut_ptr();
        let wet_r  = wetblock.r.as_mut_ptr();

        let ms_m   = ms.m.as_mut_ptr();
        let ms_s   = ms.s.as_mut_ptr();

        let data_l = data_l.as_mut_ptr();
        let data_r = data_r.as_mut_ptr();

        unsafe {
            encode_mid_side(
                wet_l, 
                wet_r, 
                ms_m, 
                ms_s, 
                BLOCK_SIZE_QUAD);

            self.width.multiply_block(
                ms_s, 
                BLOCK_SIZE_QUAD);

            decode_mid_side(
                ms_m, 
                ms_s, 
                wet_l, 
                wet_r, 
                BLOCK_SIZE_QUAD);

            self.mix.fade_2_blocks_to(
                data_l, 
                wet_l, 
                data_r, 
                wet_r, 
                data_l, 
                data_r, 
                BLOCK_SIZE_QUAD);
        }
    }
}
