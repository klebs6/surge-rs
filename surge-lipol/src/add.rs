ix!();

impl crate::LipolPs {

    /// # Safety
    ///
    /// caller needs to ensure we can access nquads 
    /// valid items contiguously from a valid src pointer
    pub unsafe fn add_block<NQ: TryInto<usize>>(
        &mut self, 
        src: *mut f32, 
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |idx:usize, ref mut y1, ref mut y2, ref mut dy| {

                let src = src as *mut __m128;

                *src.add(idx) =  _mm_add_ps(*src.add(idx), *y1);

                *y1 =  _mm_add_ps(*y1, *dy) ;

                *src.add(idx + 1) =  _mm_add_ps(*src.add(idx + 1), *y2);

                *y2 =  _mm_add_ps(*y2, *dy);
            }
        );
    }

    /// # Safety
    ///
    /// caller needs to ensure we can access nquads 
    /// valid items contiguously from a valid src pointer
    pub unsafe fn subtract_block<NQ: TryInto<usize>>(
        &mut self, 
        src: *mut f32, 
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: Debug
    {
        let nquads: usize = nquads.try_into().unwrap();

        lipol_ps_sse_block!(self, nquads, 2; 
            |idx:usize, ref mut y1, ref mut y2, ref mut dy| {

                let src = src as *mut __m128;

                *src.add(idx) = _mm_sub_ps(
                    *src.add(idx), 
                    *y1
                );

                *y1 = _mm_add_ps(*y1, *dy);

                *src.add(idx + 1) = _mm_sub_ps(
                    *src.add(idx + 1), 
                    *y2
                );

                *y2 = _mm_add_ps(*y2, *dy);
            }
        );
    }
}
