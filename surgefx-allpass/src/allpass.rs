crate::ix!();

#[derive(Debug,Clone)]
pub struct Allpass {
    len:  usize,
    k:    usize,
    data: A1d::<f32>,
}

impl Default for Allpass { fn default() -> Self { 
    Self { 
        k: 0, 
        len: 1, 
        data: A1d::<f32>::zeros( ALLPASS_REVERB_MAX_ALLPASS_LEN ), 
    } 
}}

impl Allpass {

    pub fn set_len(&mut self, len: usize) { self.len = len; }

    pub fn process(&mut self, input: f32, coeff: f32) -> f32 {

        self.k += 1;

        if self.k >= self.len {
            self.k = 0;
        }

        let delay_in: f32 = input - 
            coeff * self.data[self.k];

        let result:   f32 = self.data[self.k] + 
            coeff * delay_in;

        self.data[self.k] =   delay_in;

        result
    }
}

