crate::ix!();

impl WindowOscillator {

    pub fn process_sub_osc(&mut self, so: usize, subosc_cfg: &SubOscProcessCfg) {

        let SubOscProcessCfg { table, formant_mul, window, fm, .. } = subosc_cfg;

        let mut pos: u32 = self.pos[so];

        let mut ratio_a: u32 = self.ratio[so] as u32;

        if *fm {
            ratio_a = self.fm_ratio[[so,0]] as u32;
        }

        let mipmap_a: u32;
        let mipmap_b: u32;

        let wave_wavetable_num_tables = self.wave_wavetable.num_tables() as u32;

        if self.table[so] >= wave_wavetable_num_tables {
            self.table[so] = *table as u32;
        }

        /*
         | table_id may not be valid anymore if
         | a new wavetable is loaded
         |
         */
        let bs: u32 = {

            let t0 = 3 * formant_mul;

            big_mul_r16(ratio_a, t0 as u32)
        };

        let mut msb_pos: u32 = bitscan_reverse(bs);

        mipmap_b = {

            let t0 = msb_pos as i32;
            let t1 = self.wave_wavetable.num_samples_per_table_po2() - 1;

            limit_range(
                t0 - 17, 
                0, 
                t1 as i32
            ) as u32
        };

        msb_pos = bitscan_reverse(3 * ratio_a);

        mipmap_a = {

            let t0 = (msb_pos as i32) - 17;
            let t1 = self.window_wavetable.num_samples_per_table_po2() - 1;

            limit_range(
                t0, 
                0, 
                t1 as i32
            ) as u32
        };

        //check this -- possibly broken

        let wave_table_idx:  usize = self.table[so].try_into().unwrap();
        let wave_mipmap_idx: usize = mipmap_b.try_into().unwrap();

        let win_table_idx:   usize = (*window).try_into().unwrap();
        let win_mipmap_idx:  usize = mipmap_a.try_into().unwrap();

        let wave_adr: *const i16 = {
            self.wave_wavetable
                .data
                .index_axis(Axis(0),wave_mipmap_idx)
                .index_axis(Axis(0),wave_table_idx)
                .as_ptr()
        };

        let win_adr: *const i16 = {
            self.window_wavetable
                .data
                .index_axis(Axis(0),win_mipmap_idx as usize)
                .index_axis(Axis(0),win_table_idx)
                .as_ptr()
        };

        /*
        let mut wave_adr: *mut i16 = (self.wave_wavetable.table_i16_weak_ptrs[mipmap_b as usize][self.table[so] as usize] as *mut i32) as *mut i16;
        let win_adr: *mut i16 = (self.window_wavetable.table_i16_weak_ptrs[mipmap_a as usize][window as usize] as *mut i32)  as *mut i16;
        */
        let subosc_block_cfg = SubOscProcessBlockCfg {
            ratio_a,
            mipmap_a,
            mipmap_b,
            wave_mipmap_idx,
            wave_adr,
            win_adr,
        };

        for i in (0_usize..BLOCK_SIZE_OS as usize).step_by(1) {
            unsafe {
                self.process_oscillator_block(so,i,&mut pos,&subosc_cfg,&subosc_block_cfg);
            }
        }

        self.pos[so] = pos;
    }
}
