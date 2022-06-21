pub const WINDOW_OSCILLATOR_NUM_SUBOSCS: usize = 2;

pub struct SubOscProcessCfg {
    pub table:         i32,
    pub formant_mul:   i32,
    pub size_mask:     u32,
    pub size_mask_win: u32,
    pub window:        i8,
    pub stereo:        bool,
    pub fm:            bool,
}

pub struct SubOscProcessBlockCfg {
    pub ratio_a:         u32,
    pub mipmap_a:        u32,
    pub mipmap_b:        u32,
    pub wave_mipmap_idx: usize,
    pub wave_adr:        *const i16,
    pub win_adr:         *const i16,
}
