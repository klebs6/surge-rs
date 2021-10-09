ix!();

pub const N_VOCODER_BANDS: usize = 20;
pub const N_VOCODER_VEC:   usize = N_VOCODER_BANDS >> 2;

// HZ from http://www.sequencer.de/pix/moog/moog_vocoder_rack.jpg
pub const VOCODER_FREQ_VSM201: [f32; N_VOCODER_BANDS] = [
    180.0,  
    219.0,  
    266.0,  
    324.0,  
    394.0,  
    480.0,  
    584.0,
    711.0,  
    865.0,  
    1053.0, 
    1281.0, 
    1559.0, 
    1898.0, 
    2309.0,
    2810.0, 
    3420.0, 
    4162.0, 
    5064.0, 
    6163.0, 
    7500.0
    ];

#[inline] pub fn vocoder_default_freq_low() -> f32 {
    12.0 * (VOCODER_FREQ_VSM201[0] / (CONCERT_A_HZ as f32)).log2()
}

#[inline] pub fn vocoder_default_freq_high() -> f32 {
    12.0 * (VOCODER_FREQ_VSM201[N_VOCODER_BANDS-1] / (CONCERT_A_HZ as f32)).log2()
}
