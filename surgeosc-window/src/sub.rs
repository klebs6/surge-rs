ix!();

use crate::{
    WindowOscillator,
    WindowOscillatorParam,
};

impl WindowOscillator<'sr> {

    fn get_formant_mul(&self, formant: f64) -> i32 {

        let mut formant_mul: i32 = 
            ((65536.0 * self.tuner.n2p_tuningctr(formant as f64)) as f32) as i32;

        // We can actually get input tables bigger than the convolution table
        let window_vs_wave_po2: i32 = (self.window_wavetable.num_samples_per_table_po2() - self.wave_wavetable.num_samples_per_table_po2()) as i32;

        if window_vs_wave_po2 < 0 {
            formant_mul = maxi(formant_mul << -window_vs_wave_po2, 1);

        } else {
            formant_mul = maxi(formant_mul >> window_vs_wave_po2, 1);
        }

        formant_mul
    }

    pub fn process_sub_oscs(&mut self, stereo: bool, fm: bool) {
        // SSE2 path
        let morph = self.pvalf(WindowOscillatorParam::Morph);

        let table: i32 = limit_range_i(
            ((self.wave_wavetable.num_tables() as f32 * morph) as f32) as i32, 
            0, (self.wave_wavetable.num_tables() as i32) - 1);

        let formant = self.pvalf(WindowOscillatorParam::Formant);
        let formant_mul = self.get_formant_mul(formant as f64);

        let size_mask:    u32 = ((self.wave_wavetable.num_samples_per_table() as u32) << 16) - 1;
        let size_mask_win: u32 = ((self.window_wavetable.num_samples_per_table() << 16) as u32) - 1;

        let window: i8 = limit_range_i(
            self.pvali(WindowOscillatorParam::Window),
            0, 8) as i8;

        for so in (0_usize..self.active_sub_oscs as usize).step_by(1) {

            let mut pos: u32 = self.pos[so];

            let mut ratio_a: u32 = self.ratio[so] as u32;

            if fm {
                ratio_a = self.fm_ratio[[so,0]] as u32;
            }

            let mipmap_a: u32;
            let mipmap_b: u32;

            if self.table[so] >= (self.wave_wavetable.num_tables() as u32) {
                self.table[so] = table as u32;
            }

            // table_id may not be valid anymore if a new wavetable is loaded

            let bs: u32 = big_mul_r16(ratio_a, (3 * formant_mul) as u32);

            let mut msb_pos: u32 = bitscan_reverse(bs);
            mipmap_b = limit_range_i((msb_pos as i32) - 17, 0, (self.wave_wavetable.num_samples_per_table_po2() - 1) as i32) as u32;

            msb_pos = bitscan_reverse(3 * ratio_a);
            mipmap_a = limit_range_i(
                (msb_pos as i32) - 17, 
                0, 
                (self.window_wavetable.num_samples_per_table_po2() - 1) as i32
            ) as u32;

            //check this -- possibly broken

            let wave_table_idx:  usize = self.table[so].try_into().unwrap();
            let wave_mipmap_idx: usize = mipmap_b.try_into().unwrap();

            let win_table_idx:  usize = window.try_into().unwrap();
            let win_mipmap_idx: usize = mipmap_a.try_into().unwrap();

            let mut wave_adr: *const i16 = self.wave_wavetable.data.index_axis(Axis(0),wave_mipmap_idx).index_axis(Axis(0),wave_table_idx).as_ptr();
            let win_adr:  *const i16 = self.window_wavetable.data.index_axis(Axis(0),win_mipmap_idx as usize).index_axis(Axis(0),win_table_idx).as_ptr();
                /*
            let mut wave_adr: *mut i16 = (self.wave_wavetable.table_i16_weak_ptrs[mipmap_b as usize][self.table[so] as usize] as *mut i32) as *mut i16;
            let win_adr: *mut i16 = (self.window_wavetable.table_i16_weak_ptrs[mipmap_a as usize][window as usize] as *mut i32)  as *mut i16;
                */

            for i in (0_usize..BLOCK_SIZE_OS as usize).step_by(1) {

                if fm {
                    pos += self.fm_ratio[[so,i]] as u32;
                } else {

                    pos += ratio_a;
                }

                if (pos & !size_mask_win) != 0 {

                    self.formant_mul[so] = formant_mul as u32;
                    self.table[so] = table as u32;

                    wave_adr = self.wave_wavetable.data.index_axis(Axis(0),wave_mipmap_idx).index_axis(Axis(0),table as usize).as_ptr();

                    pos &= size_mask_win;

                }

                let win_pos:   u32 = pos >> (16 + mipmap_a);
                let win_s_pos:  u32 = (pos >> (8 + mipmap_a)) & 0xFF;

                let f_pos:     u32 = big_mul_r16(self.formant_mul[so], pos) & size_mask;

                let m_pos:     u32 = f_pos >> (16 + mipmap_b);
                let ms_pos:    u32 = (f_pos >> (8 + mipmap_b)) & 0xFF;

                unsafe {
                    let wave: __m128i = _mm_madd_epi16(
                        _mm_load_si128(
                            (self.tables.sinctable_i16_ptr(0) as *const __m128i).add(ms_pos.try_into().unwrap())
                        ),
                        _mm_loadu_si128(
                            (wave_adr as *const __m128i).add(m_pos.try_into().unwrap()))
                    );

                    let win: __m128i = _mm_madd_epi16(
                        _mm_load_si128((self.tables.sinctable_i16_ptr(0) as *const __m128i).add(win_s_pos.try_into().unwrap())),
                        _mm_loadu_si128((win_adr as *const __m128i).add(win_pos.try_into().unwrap()))
                    );

                    // Sum
                    let mut i_win  = WetBlock1::<4>::default();
                    let mut i_wave = WetBlock1::<4>::default();

                    if cfg![macos] {
                        // this should be very fast on C2D/C1D (and there are no macs with K8's)
                        i_win.buf[0]  = _mm_cvtsi128_si32(win) as f32;
                        i_win.buf[1]  = _mm_cvtsi128_si32(_mm_shuffle_epi32(win, _MM_SHUFFLE(1, 1, 1, 1))) as f32;
                        i_win.buf[2]  = _mm_cvtsi128_si32(_mm_shuffle_epi32(win, _MM_SHUFFLE(2, 2, 2, 2))) as f32;
                        i_win.buf[3]  = _mm_cvtsi128_si32(_mm_shuffle_epi32(win, _MM_SHUFFLE(3, 3, 3, 3))) as f32;
                        i_wave.buf[0] = _mm_cvtsi128_si32(wave) as f32;
                        i_wave.buf[1] = _mm_cvtsi128_si32(_mm_shuffle_epi32(wave, _MM_SHUFFLE(1, 1, 1, 1))) as f32;
                        i_wave.buf[2] = _mm_cvtsi128_si32(_mm_shuffle_epi32(wave, _MM_SHUFFLE(2, 2, 2, 2))) as f32;
                        i_wave.buf[3] = _mm_cvtsi128_si32(_mm_shuffle_epi32(wave, _MM_SHUFFLE(3, 3, 3, 3))) as f32;
                    } else {
                        _mm_store_si128(i_win.buf.as_mut_ptr() as *mut __m128i,  win);
                        _mm_store_si128(i_wave.buf.as_mut_ptr() as *mut __m128i, wave);
                    }

                    i_win.buf[0]  = (((i_win.buf[0] + i_win.buf[1] + i_win.buf[2] + i_win.buf[3]) as i32) >> 13) as f32;
                    i_wave.buf[0] = (((i_wave.buf[0] + i_wave.buf[1] + i_wave.buf[2] + i_wave.buf[3]) as i32) >> 13) as f32;

                    if stereo {
                        let out: i32 = (((i_win.buf[0] * i_wave.buf[0]) as i32) >> 7) as i32;
                        self.out.l[i] += ((out * (self.gain[[so,0]] as i32)) >> 6) as f32;
                        self.out.r[i] += ((out * (self.gain[[so,1]] as i32)) >> 6) as f32;
                    } else {
                        self.out.l[i] += (((i_win.buf[0] * i_wave.buf[0]) as i32) >> 6) as f32;
                    }
                }
            }
            self.pos[so] = pos;
        }
    }
}

