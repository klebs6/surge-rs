ix!();

use crate::{
    LipolPs,
};

impl LipolPs {

    /// panning that always lets both channels through unattenuated 
    /// (seperate hard-panning)
    ///
    /// # Safety
    ///
    /// need to make sure we can access nquads blocks
    /// safely from left, right, d_l, and d_r
    pub unsafe fn trixpan_blocks<NQ>(
        &mut self,
        left:    *mut f32,
        right:   *mut f32,
        d_l:     *mut f32,
        d_r:     *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        let mut y: __m128 = z128![];
        let mut dy: __m128 = z128![];
        self.initblock(&mut y, &mut dy);

        for idx in 0..nquads 
        {
            let a: __m128 = _mm_max_ps(z128![], y);
            let b: __m128 = _mm_min_ps(z128![], y);

            let t_l: __m128 = _mm_sub_ps(
                _mm_mul_ps(
                    _mm_sub_ps(m128_one![], a), 
                    *(left as *mut __m128).add(idx)
                ),
                _mm_mul_ps(
                    b, 
                    *(right as *mut __m128).add(idx)
                )
            ); // left = (1-a)*left - b*right

            let t_r: __m128 = _mm_add_ps(
                _mm_mul_ps(a, *(left as *mut __m128).add(idx)), 
                _mm_mul_ps(
                    _mm_add_ps(m128_one![], b), 
                    *(right as *mut __m128).add(idx)
                )
            ); // right = a*left + (1+b)*right

            *(d_l as *mut __m128).add(idx) = t_l;
            *(d_r as *mut __m128).add(idx) = t_r;

            y = _mm_add_ps(y, dy);
        }
    }
}
