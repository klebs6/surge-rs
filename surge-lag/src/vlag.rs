ix!();

pub const VLAG_MIN_BW:   f64   = 0.0001;
pub const VLAG_D_LP:     f64   = 0.004;
pub const VLAG_D_LPINV:  f64   = 1.0 - 0.004;

pub type VDouble  = [f64; 2];

#[derive(Debug,Clone)]
pub struct VLag {
    pub v:        Align16<VDouble>,
    pub target_v: Align16<VDouble>,
}

impl Default for VLag {
    fn default() -> Self {
        Self::new_x87()
    }
}

impl VLag {

    #[inline] pub fn new_x87() -> Self {
        Self { 
            v:        Align16([0.0; 2]),
            target_v: Align16([0.0; 2]),
        }
    }

    #[inline] pub fn init_x87(&mut self) {
        self.v[0] = 0.0;
        self.v[1] = 0.0;
        self.target_v[0] = 0.0;
        self.target_v[1] = 0.0;
    }

    #[inline] pub fn process(&mut self) {
        self.v[0] = self.v[0] * VLAG_D_LPINV + self.target_v[0] * VLAG_D_LP;
    }

    #[inline] pub fn new_value(&mut self, f: f64) {
        self.target_v[0] = f;
    }

    #[inline] pub fn instantize(&mut self) {
        self.v = self.target_v.clone();
    }

    #[inline] pub fn start_value(&mut self, f: f64) {
        self.target_v[0] = f;
        self.v[0] = f;
    }

}
