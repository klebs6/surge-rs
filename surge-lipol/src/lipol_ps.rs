ix!();

/// lipol_ps is a small utility class for generating small line segments
/// between values
///
/// usage would be
///
/// ```
/// lipol_ps mypol;
///
/// ...
/// 
/// mypol.set_target(13.23);
/// if init {
///   mypol.instantize();
/// ```
///
/// then later in the code
///
/// ```
/// float values alignas(16)[SIZE]
/// mypol.store_block(values, SIZE_OVER_FOUR);
/// ```
///
/// and block would contain the linear interpolation between the 
/// last queried value and the target.
///
#[derive(Debug,Clone)]
pub struct LipolPs {
    target:            __m128,
    currentval:        __m128,
    coef:              __m128,
    coef_m1:           __m128,
    lipol_block_size:  __m128,
    m128_lipolstarter: __m128,
    m128_bs4_inv:      __m128,

}

impl Default for LipolPs {
    fn default() -> Self {
        unsafe {
            let coef             =  _mm_set1_ps(0.25);
            let lipol_block_size = _mm_cvt_si2ss(z128![], 64);

            Self {
                target:             _mm_setzero_ps(),
                currentval:         _mm_setzero_ps(),
                coef,
                coef_m1:            _mm_sub_ss(m128_one![], coef),
                lipol_block_size,
                m128_lipolstarter:  _mm_set_ps(1.0, 0.75, 0.5, 0.25),
                m128_bs4_inv:       _mm_div_ss(m128_four![], lipol_block_size),
            }
        }
    }
}

impl LipolPs {

    pub fn new() -> Self {
        Self {
            .. Default::default()
        }
    }

    pub fn new_with_blocksize(n: usize) -> Self {
        let mut x = Self::new();
        x.set_blocksize(n as i32);
        x
    }

    #[inline] pub fn set_blocksize(&mut self, bs: i32) {
        unsafe {
            self.lipol_block_size = _mm_cvt_si2ss(z128![], bs);
            self.m128_bs4_inv     = _mm_div_ss(m128_four![], self.lipol_block_size);
        }
    }

    #[inline] pub fn set_target(&mut self, t: f32) {
        self.currentval = self.target;
        unsafe {
            self.target = _mm_set_ss(t);
        }
    }

    #[inline] pub fn set_target_m128(&mut self, t: __m128)
    {
        self.currentval = self.target;
        self.target = t;
    }

    #[inline] pub fn set_target_instantize(&mut self, t: f32)
    {
        self.target = unsafe{ _mm_set_ss(t) };
        self.currentval = self.target;
    }

    #[inline] pub fn set_target_smoothed(&mut self, t: f32)
    {
        self.currentval = self.target;
        unsafe {
            let p1: __m128 = _mm_mul_ss(self.coef, _mm_set_ss(t));
            let p2: __m128 =  _mm_mul_ss(self.coef_m1, self.target);
            self.target = _mm_add_ss(p1, p2);
        }
    }

    #[inline] pub fn instantize(&mut self) {
        self.currentval = self.target;
    }

    #[inline] pub fn get_target(&self) -> f32 {
        let mut f: f32 = 0.0;
        unsafe{ _mm_store_ss(&mut f, self.target) };
        f
    }

    #[inline] pub fn initblock(&mut self, 
        y: &mut __m128, 
        dy: &mut __m128) 
    {
        unsafe {

            *dy = _mm_sub_ss(
                self.target, 
                self.currentval
            );

            *dy = _mm_mul_ss(
                *dy, 
                self.m128_bs4_inv
            );

            *dy = _mm_shuffle_ps(
                *dy, 
                *dy, 
                _MM_SHUFFLE(0, 0, 0, 0)
            );

            *y  = _mm_shuffle_ps(
                self.currentval, 
                self.currentval, 
                _MM_SHUFFLE(0, 0, 0, 0)
            );

            *y  = _mm_add_ps(
                *y, 
                _mm_mul_ps( *dy, self.m128_lipolstarter)
            );
        }
    }

    /// # Safety
    ///
    /// need to be able to access nquads blocks safely and contiguously
    /// from dst
    pub unsafe fn store_block<NQ>(&mut self, 
        dst: *mut f32, nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 8; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {
                _mm_store_ps(dst.add(i), *y1);

                *y1 =  _mm_add_ps(*y1, *dy);

                _mm_store_ps(dst.add(i + 4), *y2);

                *y2 =  _mm_add_ps(*y2, *dy);
            }
        );
    }
}
