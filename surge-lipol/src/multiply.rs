ix!();

impl crate::LipolPs {

    /// # Safety
    ///
    /// need to make sure we can access nquads blocks
    /// safely from src
    pub unsafe fn multiply_block_to<NQ>(
        &mut self,
        src: *mut f32,
        dst: *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let src = src as *mut __m128;
                let dst = dst as *mut __m128;

                let a: __m128 = _mm_mul_ps(*src.add(i) , *y1);

                *dst.add(i) = a;

                *y1 = _mm_add_ps(*y1, *dy);

                let b: __m128 = _mm_mul_ps(*src.add(i + 1), *y2);

                *dst.add(i + 1) = b;

                *y2 = _mm_add_ps(*y2, *dy);
            }
        );
    }

    /// # Safety
    ///
    /// need to make sure we can access nquads blocks
    /// safely from src
    pub unsafe fn multiply_2_blocks_to<NQ>(
        &mut self,
        src1: *mut f32,
        src2: *mut f32,
        dst1: *mut f32,
        dst2: *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let src1 = src1 as *mut __m128;
                let src2 = src2 as *mut __m128;
                let dst1 = dst1 as *mut __m128;
                let dst2 = dst2 as *mut __m128;

                *dst1.add(i) = _mm_mul_ps(*src1.add(i), *y1);
                *dst2.add(i) = _mm_mul_ps(*src2.add(i), *y1);

                *y1 = _mm_add_ps(*y1, *dy);

                *dst1.add(i + 1) = _mm_mul_ps(*src1.add(i + 1), *y2);
                *dst2.add(i + 1) = _mm_mul_ps(*src2.add(i + 1), *y2);

                *y2 = _mm_add_ps(*y2, *dy);
            }
        );
    }

    /// # Safety
    ///
    /// need to make sure we can access nquads blocks
    /// safely from src
    pub unsafe fn multiply_2_blocks<NQ: TryInto<usize>>(
        &mut self,
        src1: *mut f32, //__restrict
        src2: *mut f32, //__restrict
        nquads: NQ)
        where <NQ as TryInto<usize>>::Error: Debug
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let src1 = src1 as *mut __m128;
                let src2 = src2 as *mut __m128;

                *src1.add(i) = _mm_mul_ps(*src1.add(i), *y1);
                *src2.add(i) = _mm_mul_ps(*src2.add(i), *y1);

                *y1 = _mm_add_ps(*y1, *dy);

                *src1.add(i + 1) = _mm_mul_ps(*src1.add(i + 1), *y2);
                *src2.add(i + 1) = _mm_mul_ps(*src2.add(i + 1), *y2);

                *y2 = _mm_add_ps(*y2, *dy);
            }
        );
    }

    /// # Safety
    ///
    /// need to make sure we can access nquads blocks
    /// safely from src
    pub unsafe fn multiply_block<NQ>(&mut self, 
        src: *mut f32, 
        nquads: NQ)
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 8; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let a: __m128 = _mm_mul_ps(
                    _mm_load_ps(src.add(i)), 
                    *y1
                );

                _mm_store_ps(
                    src.add(i), 
                    a);

                *y1 =  _mm_add_ps(*y1, *dy);

                let b: __m128 =  _mm_mul_ps(
                    _mm_load_ps(src.add(i + 4)), 
                    *y2
                );

                _mm_store_ps(
                    src.add(i + 4), 
                    b);

                *y2 = _mm_add_ps(*y2, *dy);
            }
            );
    }

    /// saturates the interpolator each step (for amp envelopes)
    ///
    /// # Safety
    ///
    /// need to make sure we can access nquads blocks
    /// safely from src
    pub unsafe fn multiply_block_sat1<NQ>(&mut self, 
        src: *mut f32, 
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        let satv: __m128 =  _mm_set1_ps(1.0);

        lipol_ps_sse_block!(self, nquads, 8; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                _mm_store_ps(
                    src.add(i), 
                    _mm_mul_ps(
                        _mm_load_ps(src.add(i)), 
                        *y1)
                );

                *y1 = _mm_min_ps(
                    satv, 
                    _mm_add_ps(*y1, *dy)
                );

                _mm_store_ps(
                    src.add(i + 4), 
                    _mm_mul_ps(
                        _mm_load_ps(src.add(i + 4)), *y2)
                );

                *y2 = _mm_min_ps(
                    satv, 
                    _mm_add_ps(*y2, *dy)
                );
            });
    }
}
