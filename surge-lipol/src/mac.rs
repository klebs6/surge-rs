ix!();

impl crate::LipolPs {

    /**
      | # Safety
      |
      | need to make sure we can access nquads blocks
      | safely from src and dst
      */
    pub unsafe fn mac_block_to<NQ: TryInto<usize>>(
        &mut self,
        src: *mut f32,
        dst: *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let dst = dst as *mut __m128;
                let src = src as *mut __m128;

                *dst.add(i) = 
                    _mm_add_ps(
                        *dst.add(i), 
                        _mm_mul_ps(*src.add(i), *y1)
                    );

                *y1 = _mm_add_ps(*y1, *dy);

                *dst.add(i + 1) = 
                    _mm_add_ps(
                        *dst.add(i + 1), 
                        _mm_mul_ps(*src.add(i + 1), *y2)
                    );

                *y2 = _mm_add_ps(*y2, *dy);
            });
    }

    /**
      | # Safety
      |
      | need to make sure we can access nquads blocks
      | safely from src1, src2, dst1, and dst2
      */
    pub unsafe fn mac_2_blocks_to<NQ: TryInto<usize>>(
        &mut self,
        src1: *mut f32,
        src2: *mut f32,
        dst1: *mut f32,
        dst2: *mut f32,
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |i:usize, ref mut y1, ref mut y2, ref mut dy| {

                let dst1 = dst1 as *mut __m128;
                let dst2 = dst2 as *mut __m128;

                let src1 = src1 as *mut __m128;
                let src2 = src2 as *mut __m128;

                *dst1.add(i) = 
                    _mm_add_ps(
                        *dst1.add(i), 
                        _mm_mul_ps(*src1.add(i), *y1)
                    );

                *dst2.add(i) = 
                    _mm_add_ps(
                        *dst2.add(i), 
                        _mm_mul_ps(*src2.add(i), *y1)
                    );

                *y1 = _mm_add_ps(*y1, *dy);

                *dst1.add(i + 1) = 
                    _mm_add_ps(
                        *dst1.add(i + 1), 
                        _mm_mul_ps(*src1.add(i + 1), *y2)
                    );

                *dst2.add(i + 1) = 
                    _mm_add_ps(
                        *dst2.add(i + 1), 
                        _mm_mul_ps(*src2.add(i + 1), *y2)
                    );

                *y2 = _mm_add_ps(*y2, *dy);
            });
    }
}
