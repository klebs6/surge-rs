crate::ix!();

impl crate::LipolPs {

    /// Adds the given block of contiguously-allocated `nquads` quadwords from
    /// the memory location pointed to by `src` to this `LipolPs` instance.
    ///
    /// # Safety
    ///
    /// This function is marked as `unsafe` because the caller must ensure that
    /// the memory pointed to by `src` is valid and that the `nquads` items can
    /// be accessed contiguously. 
    ///
    /// Additionally, the `src` pointer must be a valid pointer to a mutable
    /// slice of `f32` values.
    ///
    /// # Arguments
    ///
    /// * `src` - A pointer to the start of the memory region containing the quadwords to be added.
    /// * `nquads` - The number of quadwords to be added.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use std::mem::size_of;
    /// # use std::ptr::null_mut;
    /// # use crate::LipolPs;
    /// let mut lp = LipolPs::new(4);
    ///
    /// // create a block of 4 quadwords
    /// let mut block = vec![0.0f32; 16];
    ///
    /// // set the values of the first quadword
    /// block[0] = 1.0f32;
    /// block[1] = 2.0f32;
    /// block[2] = 3.0f32;
    /// block[3] = 4.0f32;
    ///
    /// // add the block to `lp`
    /// unsafe {
    ///     lp.add_block(block.as_mut_ptr(), 4);
    /// }
    ///
    /// // verify that the values were added correctly
    /// let mut result = vec![0.0f32; 16];
    /// lp.as_slice_mut().read_exact(&mut result).unwrap();
    /// assert_eq!(result[0], 1.0f32);
    /// assert_eq!(result[1], 2.0f32);
    /// assert_eq!(result[2], 3.0f32);
    /// assert_eq!(result[3], 4.0f32);
    /// ```
    ///
    /// # Safety
    ///
    /// caller needs to ensure we can access nquads 
    /// valid items contiguously from a valid src pointer
    ///
    pub unsafe fn add_block<NQ: TryInto<usize>>(
        &mut self, 
        src: *mut f32, 
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: fmt::Debug
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
    /// Caller needs to ensure that we can access nquads valid items
    /// contiguously from a valid src pointer.
    ///
    /// Subtracts the corresponding values from the current state for a block of
    /// `nquads` items in `src`, using vectorized SSE instructions. The result
    /// is stored back in `src`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use std::mem::MaybeUninit;
    /// # use std::ptr;
    /// # use lipol::LipolPs;
    /// # fn main() {
    /// let mut lipol = LipolPs::new();
    /// let mut data: [f32; 8] = unsafe { MaybeUninit::uninit().assume_init() };
    /// let nquads = data.len() / 2;
    /// data.copy_from_slice(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
    ///
    /// unsafe {
    ///     lipol.subtract_block(data.as_mut_ptr(), nquads);
    /// }
    ///
    /// assert_eq!(data, [-1.0, 1.0, -1.0, 3.0, -1.0, 5.0, -1.0, 7.0]);
    /// # }
    /// ```
    ///
    /// # Safety
    ///
    /// caller needs to ensure we can access nquads 
    /// valid items contiguously from a valid src pointer
    ///
    pub unsafe fn subtract_block<NQ: TryInto<usize>>(
        &mut self, 
        src: *mut f32, 
        nquads: NQ) 
        where <NQ as TryInto<usize>>::Error: fmt::Debug
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
