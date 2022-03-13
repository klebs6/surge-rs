ix!();

use crate::*;


impl LipolPs {

    /**
      | left = (1-a)*left - b*right
      |
      */
    #[inline] unsafe fn trixpan_left(
        a:     &__m128,
        b:     &__m128,
        left:  *const __m128,
        right: *const __m128) -> __m128 {

        let one = m128_one![];

        _mm_sub_ps(
            _mm_mul_ps(
                _mm_sub_ps(one, *a), 
                *left
            ),
            _mm_mul_ps(
                *b, 
                *right
            )
        )
    }

    /**
      | right = a*left + (1+b)*right
      |
      */
    #[inline] unsafe fn trixpan_right(
        a:     &__m128,
        b:     &__m128,
        left:  *const __m128,
        right: *const __m128) -> __m128 {

        let one = m128_one![];

        _mm_add_ps(
            _mm_mul_ps(
                *a, 
                *left
            ), 
            _mm_mul_ps(
                _mm_add_ps(one, *b), 
                *right
            )
        )
    }

    #[inline] unsafe fn trixpan_blocks_do_quad(
        left:  *mut __m128,
        right: *mut __m128,
        d_l:   *mut __m128,
        d_r:   *mut __m128,
        y:     &mut __m128,
        dy:    &mut __m128) {

        let a: __m128 = _mm_max_ps(z128![], *y);
        let b: __m128 = _mm_min_ps(z128![], *y);

        *d_l = Self::trixpan_left(&a, &b,left,right);
        *d_r = Self::trixpan_right(&a,&b,left,right);

        *y = _mm_add_ps(*y, *dy);
    }

    /// panning that always lets both channels
    /// through unattenuated (seperate
    /// hard-panning)
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
        #[inline] unsafe fn access(x: *mut f32, idx: usize) -> *mut __m128 {
            (x as *mut __m128).add(idx)
        }

        let nquads: usize = nquads.try_into().unwrap();

        let mut y:  __m128 = z128![];
        let mut dy: __m128 = z128![];

        self.initblock(&mut y, &mut dy);

        for idx in 0..nquads 
        {
            let left  = access(left,idx);
            let right = access(right,idx);
            let d_l   = access(d_l,idx);
            let d_r   = access(d_r,idx);

            Self::trixpan_blocks_do_quad(
                left,
                right,
                d_l,
                d_r,
                &mut y,
                &mut dy
            )
        }
    }
}
