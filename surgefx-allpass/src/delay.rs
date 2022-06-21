crate::ix!();

#[derive(Debug,Getters,Setters,Clone)]
pub struct AllpassDelay {

    #[getset(set = "pub")] 
    len: i32,

    k: i32,
    data: A1d::<f32>,
}

impl Default for AllpassDelay {
    fn default() -> Self {
        Self {
            k: 0,
            len: 1,
            data: A1d::<f32>::zeros( ALLPASS_REVERB_MAX_DELAY_LEN ),
        }
    }
}

impl AllpassDelay {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn process(&mut self, input: f32, tap1: i32, tap2: i32, modulation: i32) -> (f32, f32, f32) {

       self.k = (self.k + 1) & (ALLPASS_REVERB_DELAY_LEN_MASK as i32);

       let idx1:                 i32 = (self.k - tap1 ) & (ALLPASS_REVERB_DELAY_LEN_MASK as i32);
       let idx2:                 i32 = (self.k - tap2 ) & (ALLPASS_REVERB_DELAY_LEN_MASK as i32);

       let tap_out1:             f32 = self.data[idx1 as usize];
       let tap_out2:             f32 = self.data[idx2 as usize];

       let modulation_int:       i32 = modulation >> ALLPASS_REVERB_DELAY_SUBSAMPLE_BITS;
       let modulation_frac1:     i32 = modulation & (ALLPASS_REVERB_DELAY_SUBSAMPLE_RANGE as i32 - 1);
       let modulation_frac2:     i32 = (ALLPASS_REVERB_DELAY_SUBSAMPLE_RANGE as i32).saturating_sub(modulation_frac1);

       let idx1:                 i32 = (self.k - self.len + modulation_int + 1) & (ALLPASS_REVERB_DELAY_LEN_MASK as i32);
       let idx2:                 i32 = (self.k - self.len + modulation_int) & (ALLPASS_REVERB_DELAY_LEN_MASK as i32);

       let d1:                   f32 = self.data[idx1 as usize];
       let d2:                   f32 = self.data[idx2 as usize];
       let multiplier:           f32 = 1.0 / (ALLPASS_REVERB_DELAY_SUBSAMPLE_RANGE as f32);

       let result:               f32 = (d1 * modulation_frac1 as f32 + d2 * modulation_frac2 as f32) * multiplier;

       self.data[self.k as usize] = input;

       (result, tap_out1, tap_out2)
    }
}
