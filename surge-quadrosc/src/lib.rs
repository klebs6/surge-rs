#[derive(Debug,Clone)]
pub struct QuadrOsc {
    pub r: f64,
    pub i: f64,
    dr:    f64,
    di:    f64,
}

pub trait SetRate {
    fn set_rate(&mut self, w: f64);
}

pub trait SetPhase {
    fn set_phase(&mut self, w: f64);
}

pub trait Process {
    fn process(&mut self);
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

impl SetRate for QuadrOsc {

    #[inline] fn set_rate(&mut self, w: f64) {

        self.dr = w.cos();
        self.di = w.sin();

        let n: f64 = 1.0 / ( 
            self.r * self.r + self.i * self.i
        ).sqrt();

        self.r *= n;
        self.i *= n;
    }
}

impl SetPhase for QuadrOsc {

    #[inline] fn set_phase(&mut self, w: f64) {
        self.r = w.sin();
        self.i = - w.cos();
    }
}

impl Process for QuadrOsc {

    #[inline] fn process(&mut self) {
        let lr: f64 = self.r;
        let li: f64 = self.i;
        self.r = self.dr * lr - self.di * li;
        self.i = self.dr * li + self.di * lr;
    }
}
