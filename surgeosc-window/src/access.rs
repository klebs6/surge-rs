crate::ix!();

impl WindowOscillator {

    #[inline] pub fn get_table(&self, morph: f32) -> i32 {

        let t0 = self.wave_wavetable.num_tables() as i32;
        let t1 = self.wave_wavetable.num_tables() as f32;

        let t2 = t1 * morph;
        let t3 = (t2 as f32) as i32;

        limit_range(t3, 0, t0 - 1)
    }

    #[inline] pub fn get_size_mask(&self) -> u32 {
        let t0 = self.wave_wavetable.num_samples_per_table();
        ((t0 as u32) << 16) - 1
    }

    #[inline] pub fn get_size_mask_win(&self) -> u32 {
        let t0 = self.window_wavetable.num_samples_per_table();
        ((t0 << 16) as u32) - 1
    }

    #[inline] pub fn get_window(&self) -> i8 {
        let t0 = self.pvali(WindowOscillatorParam::Window);
        limit_range(t0, 0, 8) as i8
    }

    #[inline] pub unsafe fn get_wave(&self, 
        ms_pos:   u32, 
        m_pos:    u32, 
        wave_adr: *const i16) -> __m128i {

        let p0 = self.tables.sinctable_i16_ptr(0) as *const __m128i;
        let p1 = p0.add(ms_pos.try_into().unwrap());
        let p2 = _mm_load_si128(p1);

        let p3 = *wave_adr as *const __m128i;
        let p4 = p3.add(m_pos.try_into().unwrap());
        let p5 = _mm_loadu_si128(p4);

        _mm_madd_epi16(p2, p5)
    }

    #[inline] pub unsafe fn get_win(&self, 
        win_s_pos: u32, 
        win_pos:   u32, 
        win_adr:   *const i16) -> __m128i 
    {
        let p0 = self.tables.sinctable_i16_ptr(0) as *const __m128i;
        let p1 = p0.add(win_s_pos.try_into().unwrap());

        let p2 = *win_adr as *const __m128i;

        let p3 = p2.add(win_pos.try_into().unwrap());

        let p4 = _mm_load_si128(p1);
        let p5 = _mm_loadu_si128(p3);

        _mm_madd_epi16(p4, p5)
    }
}
