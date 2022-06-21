crate::ix!();

const FIRST_RUN_CHECKS: bool = true;

#[derive(Debug,Clone)]
pub struct LiPol<T: Num + From<f32> + Clone + AddAssign> {
    pub v:     T,
    pub new_v: T,
    pub dv:    T,
    bs_inv:    T,
    first_run: bool,
}

impl<T: Num + From<f32> + Clone + AddAssign > Default for LiPol<T> {
    fn default() -> Self {
        Self::new(BLOCK_SIZE)
    }
}

impl<T: Num + From<f32> + Clone + AddAssign > LiPol<T> {

    pub fn new(blocksize: usize) -> Self {
        let mut x = Self {
            v:         T::zero(),
            new_v:     T::zero(),
            dv:        T::zero(),
            bs_inv:    T::zero(),
            first_run: true,
        };
        x.set_blocksize(blocksize);
        x
    }

    #[inline] pub fn set_blocksize(&mut self, n: usize) {
        self.bs_inv = (1.0 / n as f32).into();
    }

    #[inline] pub fn instantize(&mut self) {
        self.v  = self.new_v.clone();
        self.dv = T::zero();
    }

    #[inline] pub fn process(&mut self) {
        self.v += self.dv.clone();
    }

    #[inline] pub fn get_target_value(&mut self) -> T {
        self.new_v.clone()
    }

    #[inline] pub fn reset(&mut self, blocksize: usize) {

        self.first_run = true;
        self.new_v     = T::zero();
        self.v         = T::zero();
        self.dv        = T::zero();
        self.bs_inv    = T::zero();

        self.set_blocksize(blocksize);
    }

    #[inline] pub fn new_value(&mut self, x: T) {

        self.v     = self.new_v.clone();
        self.new_v = x.clone();

        if FIRST_RUN_CHECKS && self.first_run {
            self.v = x;
            self.first_run = false;
        }

        self.dv = 
            (self.new_v.clone() - self.v.clone()) * 
            self.bs_inv.clone();
    }
}
