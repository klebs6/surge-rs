crate::ix!();

impl crate::LipolPs {

    /// # Safety
    ///
    /// neet to make sure we can safely access nquads elements from 
    /// src1, src2, and dst
    pub unsafe fn fade_block_to<NQ>(
        &mut self,
        src1:   *mut f32,
        src2:   *mut f32,
        dst:    *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let dst  = dst as *mut __m128;
                let src1 = src1 as *mut __m128;
                let src2 = src2 as *mut __m128;
                let one  = m128_one![];


                let mut a: __m128 = _mm_mul_ps(
                    *src1.add(i), 
                    _mm_sub_ps(one, *y1)
                );

                let mut b: __m128 = _mm_mul_ps(
                    *src2.add(i), 
                    *y1
                );

                *dst.add(i) = _mm_add_ps(a, b);

                *y1 = _mm_add_ps(*y1, *dy);

                a = _mm_mul_ps(
                    *src1.add(i + 1), 
                    _mm_sub_ps(one, *y2)
                );

                b = _mm_mul_ps(
                    *src2.add(i + 1), 
                    *y2
                );

                *dst.add(i + 1) = _mm_add_ps(a, b);

                *y2 = _mm_add_ps(*y2, *dy);
            }
        );
    }

    /// # Safety
    ///
    /// neet to make sure we can safely access nquads elements from 
    /// src11, src12, src21, src22, dst1, and dst2
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn fade_2_blocks_to<NQ>(
        &mut self,
        src11:  *mut f32,
        src12:  *mut f32,
        src21:  *mut f32,
        src22:  *mut f32,
        dst1:   *mut f32,
        dst2:   *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug, NQ: TryInto<usize>
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let src11 = src11 as *mut __m128;
                let src12 = src12 as *mut __m128;

                let src21 = src21 as *mut __m128;
                let src22 = src22 as *mut __m128;

                let dst1 = dst1 as *mut __m128;
                let dst2 = dst2 as *mut __m128;

                let one = m128_one![];

                let mut a: __m128 = _mm_mul_ps(
                    *src11.add(i), 
                    _mm_sub_ps(one, *y1)
                );

                let mut b: __m128 = _mm_mul_ps(
                    *src12.add(i), 
                    *y1
                );

                *dst1.add(i) = _mm_add_ps(a, b);

                a = _mm_mul_ps(
                    *src21.add(i), 
                    _mm_sub_ps(one, *y1)
                );

                b = _mm_mul_ps(
                    *src22.add(i), 
                    *y1
                );

                *dst2.add(i) = _mm_add_ps(a, b);

                *y1 = _mm_add_ps(*y1, *dy);

                a = _mm_mul_ps(
                    *src11.add(i + 1), 
                    _mm_sub_ps(one, *y2)
                );

                b = _mm_mul_ps(
                    *src12.add(i + 1), 
                    *y2
                );

                *dst1.add(i + 1) = _mm_add_ps(a, b);

                a = _mm_mul_ps(
                    *src21.add(i + 1), 
                    _mm_sub_ps(one, *y2)
                );

                b = _mm_mul_ps(
                    *src22.add(i + 1), 
                    *y2
                );

                *dst2.add(i + 1) = _mm_add_ps(a, b);

                *y2 = _mm_add_ps(*y2, *dy);
            });
    }
}
