here is the file, actually
#[cfg(test)] mod tests;

#[derive(Debug,Clone)]
pub struct QuadrOsc {
    pub r: f64,
    pub i: f64,
    dr: f64,
    di: f64,
}

impl Default for QuadrOsc {
    fn default() -> Self {
        Self{
            r:  0.0,
            i: -1.0,
            dr: 0.0,
            di: 0.0,
        }
    }
}

impl QuadrOsc {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline] pub fn set_rate(&mut self, w: f64) {

        self.dr = w.cos();
        self.di = w.sin();

        let n: f64 = 1.0 / ( 
            self.r * self.r + self.i * self.i
        ).sqrt();

        self.r *= n;
        self.i *= n;
    }

    #[inline] pub fn set_phase(&mut self, w: f64) {
        self.r = w.sin();
        self.i = - w.cos();
    }

    #[inline] pub fn process(&mut self) {
        let lr: f64 = self.r;
        let li: f64 = self.i;
        self.r = self.dr * lr - self.di * li;
        self.i = self.dr * li + self.di * lr;
    }
}
