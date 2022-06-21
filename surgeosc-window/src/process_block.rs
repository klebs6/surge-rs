crate::ix!();

impl WindowOscillator {

    pub unsafe fn process_oscillator_block(&mut self, 
        so:               usize, 
        block_idx:        usize, 
        pos:              &mut u32,
        subosc_cfg:       &SubOscProcessCfg,
        subosc_block_cfg: &SubOscProcessBlockCfg)
    {
        let SubOscProcessCfg      { table, formant_mul, size_mask, size_mask_win, stereo, fm, .. }        = subosc_cfg;
        let SubOscProcessBlockCfg { ratio_a, mipmap_a, mipmap_b, wave_mipmap_idx, mut wave_adr, win_adr } = subosc_block_cfg;

        //---------------
        if *fm {
            *pos += self.fm_ratio[[so,block_idx]] as u32;
        } else {
            *pos += ratio_a;
        }

        if (*pos & !size_mask_win) != 0 {

            self.formant_mul[so] = *formant_mul as u32;
            self.table[so]       = *table as u32;

            wave_adr = {
                self.wave_wavetable
                    .data
                    .index_axis(Axis(0),*wave_mipmap_idx)
                    .index_axis(Axis(0),*table as usize)
                    .as_ptr()
            };

            *pos &= size_mask_win;
        }

        let win_pos:   u32 = *pos >> (16 + mipmap_a);
        let win_s_pos: u32 = (*pos >> (8 + mipmap_a)) & 0xFF;

        let f_pos:     u32 = big_mul_r16(self.formant_mul[so], *pos) & size_mask;

        let m_pos:     u32 = f_pos >> (16 + mipmap_b);
        let ms_pos:    u32 = (f_pos >> (8 + mipmap_b)) & 0xFF;

        let wave: __m128i = self.get_wave(ms_pos, m_pos, wave_adr);
        let win:  __m128i = self.get_win(win_s_pos, win_pos, *win_adr);

        // Sum
        let i_win  = self.create_wetblock(win);
        let i_wave = self.create_wetblock(wave);

        if *stereo {

            self.block_update_stereo_out(so, block_idx, &i_win, &i_wave);

        } else {

            self.block_update_mono_out(block_idx, &i_win, &i_wave);
        }
    }
}
