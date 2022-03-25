ix!();

use crate::*;

impl Process for Reverb {

    fn process<const N: usize>(&mut self, data_l: &mut [f32; N], data_r: &mut [f32; N]) {

        let mix   = self.pvalf(ReverbParam::Mix);
        let width = self.pvalf(ReverbParam::Width);

        let need_update_rtime = self.decaytime_diff() > 0.001;

        self.maybe_update_preset();

        if need_update_rtime {
            self.update_rtime();
        }

        self.maybe_recalc_coefficients();

        self.mix.set_target_smoothed(mix);

        self.width.set_target_smoothed(self.tables.db_to_linear(width));

        let mut wetblock = WetBlock::new(BLOCK_SIZE);

        self.do_reverb(&mut wetblock, data_l, data_r);

        self.process_filter_bands(&mut wetblock);

        self.process_mid_side(data_l, data_r, &mut wetblock);
    }
}

impl Reverb {

    #[inline] fn decaytime_diff(&self) -> f32 {

        let decay_time = self.pvalf(ReverbParam::DecayTime);

        (decay_time - self.lastf[ReverbParam::DecayTime as usize]).abs()
    }

    #[inline] fn maybe_update_preset(&mut self) 
    {
        let f_roomsize    = self.pvalf(ReverbParam::RoomSize);
        let last_roomsize = self.lastf[ReverbParam::RoomSize as usize];

        let roomsize_diff = (f_roomsize - last_roomsize).abs();

        let i_shape       = self.pvali(ReverbParam::RoomShape);

        let preset_update_requested: bool = i_shape != (self.preset as i32);
        let preset_need_update:      bool = (self.b == 0) && (roomsize_diff > 0.001);

        if preset_need_update 
        || preset_update_requested 
        {
            let new_preset = ReverbPreset::try_from(i_shape as usize).unwrap();
            self.load_preset(new_preset);
        }
    }

    fn process_mid_side<const N: usize>(
        &mut self, 
        data_l:   &mut [f32; N], 
        data_r:   &mut [f32; N],
        wetblock: &mut WetBlock) 
    {
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
