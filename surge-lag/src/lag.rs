crate::ix!();

const FIRST_RUN_CHECKS: bool = true;

#[derive(Debug,Clone)]
pub struct Lag<T: Num + From<f32> + Clone + AddAssign > {
    pub v:        T,
    pub target_v: T,
    first_run:    bool,
    lp:           T,
    lpinv:        T,
}


impl<T: Num + From<f32> + Clone + AddAssign > Default for Lag<T> {
    fn default() -> Self {
        let lp: T = 0.004.into();
        Self {
            v:         T::zero(),
            target_v:  T::zero(),
            first_run: true,
            lp:        lp.clone(),
            lpinv:     T::from(1.0) - lp
        }
    }
}

impl<T: Num + From<f32> + Clone + AddAssign > Lag<T> {

    pub fn new(lp: T) -> Self {
        Self {
            lp,
            .. Default::default()
        }
    }

    #[inline] pub fn set_rate(&mut self, lp: T) {
        self.lp    = lp.clone();
        self.lpinv = T::from(1.0) - lp;
    }

    #[inline] pub fn new_value(&mut self, x: T) {

        self.target_v = x;

        if FIRST_RUN_CHECKS && self.first_run {
            self.v = self.target_v.clone();
            self.first_run = false;
        }
    }

    #[inline] pub fn start_value(&mut self, x: T) {

        self.target_v = x.clone();
        self.v        = x;

        if FIRST_RUN_CHECKS && self.first_run {
            self.first_run = false;
        }
    }

    #[inline] pub fn instantize(&mut self) {
        self.v  = self.target_v.clone();
    }

    #[inline] pub fn process(&mut self) {
        self.v = self.v.clone() * self.lpinv.clone() + 
            self.target_v.clone() * self.lp.clone();
    }

    #[inline] pub fn get_target_value(&mut self) -> T {
        self.target_v.clone()
    }
}
