ix!();

#[derive(Debug,Clone)]
pub struct InterpDelay {
    line: [f32; FLANGER_DELAY_SIZE],
    k:    i32,
}

impl Default for InterpDelay {
    fn default() -> Self {
        Self {
            line: [0.0; FLANGER_DELAY_SIZE],
            k: 0
        }
    }
}

impl InterpDelay {

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        for i in &mut self.line[..] { *i = 0.0 }
        self.k = 0;
    }

    pub fn value(&mut self, delay_by: f32) -> f32 {

           // so if delay_by is 19.2
           let itap: i32 = delay_by as i32;           // this is 19
           let fractap: f32 = delay_by - itap as f32; // this is .2

           let k0: i32 = ( self.k + FLANGER_DELAY_SIZE as i32 - itap - 1 ) & FLANGER_DELAY_SIZE_MASK as i32; // this is 20 back
           let k1: i32 = ( self.k + FLANGER_DELAY_SIZE as i32 - itap     ) & FLANGER_DELAY_SIZE_MASK as i32; // this is 19 back

           // FIXME move to the one mul form
           let result: f32 = self.line[k0 as usize] * fractap + self.line[k1 as usize] * ( 1.0 - fractap ); 

           result
    }

    pub fn push(&mut self, nv: f32) {
        self.k = ( self.k + 1 ) & FLANGER_DELAY_SIZE_MASK as i32;
        self.line[self.k as usize] = nv;
    }
}

