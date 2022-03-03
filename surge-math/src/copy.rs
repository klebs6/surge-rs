ix!();

// copy block (requires aligned data)
pub fn copy_block<NQ>(src: *mut f32, dst: *mut f32, nquads: NQ) 
        where <NQ as std::convert::TryInto<usize>>::Error: std::fmt::Debug, NQ: TryInto<usize>
{
   let nquads: usize = nquads.try_into().unwrap();

   let fdst: *mut f32 = dst as *mut f32;
   let fsrc: *mut f32 = src as *mut f32;

   for i in (0..(nquads << 2)).step_by(8 << 2) 
   {
       unsafe{ 
           _mm_store_ps(fdst.add(i), _mm_load_ps(fsrc.add(i)));
           _mm_store_ps(fdst.add(i + 4), _mm_load_ps(fsrc.add(i + 4)));
           _mm_store_ps(fdst.add(i + 8), _mm_load_ps(fsrc.add(i + 8)));
           _mm_store_ps(fdst.add(i + 12), _mm_load_ps(fsrc.add(i + 12)));
           _mm_store_ps(fdst.add(i + 16), _mm_load_ps(fsrc.add(i + 16)));
           _mm_store_ps(fdst.add(i + 20), _mm_load_ps(fsrc.add(i + 20)));
           _mm_store_ps(fdst.add(i + 24), _mm_load_ps(fsrc.add(i + 24)));
           _mm_store_ps(fdst.add(i + 28), _mm_load_ps(fsrc.add(i + 28)));
       }
   }
}
// copy block (unaligned source)
pub fn copy_block_unaligned_source<NQ>(src: *mut f32, dst: *mut f32, nquads: NQ) 
        where <NQ as std::convert::TryInto<usize>>::Error: std::fmt::Debug, NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let fdst: *mut f32 = dst as *mut f32;
    let fsrc: *mut f32 = src as *mut f32;

    for i in (0..(nquads << 2)).step_by(8 << 2) 
    {
        unsafe{ 
            _mm_store_ps(fdst.add(i), _mm_loadu_ps(fsrc.add(i)));
            _mm_store_ps(fdst.add(i + 4), _mm_loadu_ps(fsrc.add(i + 4)));
            _mm_store_ps(fdst.add(i + 8), _mm_loadu_ps(fsrc.add(i + 8)));
            _mm_store_ps(fdst.add(i + 12), _mm_loadu_ps(fsrc.add(i + 12)));
            _mm_store_ps(fdst.add(i + 16), _mm_loadu_ps(fsrc.add(i + 16)));
            _mm_store_ps(fdst.add(i + 20), _mm_loadu_ps(fsrc.add(i + 20)));
            _mm_store_ps(fdst.add(i + 24), _mm_loadu_ps(fsrc.add(i + 24)));
            _mm_store_ps(fdst.add(i + 28), _mm_loadu_ps(fsrc.add(i + 28)));
        }
    }
}

// copy block (unaligned destination)
pub fn copy_block_unaligned_destination<NQ>(src: *mut f32, dst: *mut f32, nquads: NQ) 
        where <NQ as std::convert::TryInto<usize>>::Error: std::fmt::Debug, NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let fdst: *mut f32 = dst as *mut f32;
    let fsrc: *mut f32 = src as *mut f32;

    for i in (0..(nquads << 2)).step_by(8 << 2) 
    {
        unsafe{ 
            _mm_storeu_ps(fdst.add(i), _mm_load_ps(fsrc.add(i)));
            _mm_storeu_ps(fdst.add(i + 4), _mm_load_ps(fsrc.add(i + 4)));
            _mm_storeu_ps(fdst.add(i + 8), _mm_load_ps(fsrc.add(i + 8)));
            _mm_storeu_ps(fdst.add(i + 12), _mm_load_ps(fsrc.add(i + 12)));
            _mm_storeu_ps(fdst.add(i + 16), _mm_load_ps(fsrc.add(i + 16)));
            _mm_storeu_ps(fdst.add(i + 20), _mm_load_ps(fsrc.add(i + 20)));
            _mm_storeu_ps(fdst.add(i + 24), _mm_load_ps(fsrc.add(i + 24)));
            _mm_storeu_ps(fdst.add(i + 28), _mm_load_ps(fsrc.add(i + 28)));
        }
    }
}

// copy block (unaligned source + destination)
pub fn copy_block_unaligned_src_and_dst<NQ>(src: *mut f32, dst: *mut f32, nquads: NQ)
        where <NQ as std::convert::TryInto<usize>>::Error: std::fmt::Debug, NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let fdst: *mut f32 = dst as *mut f32;
    let fsrc: *mut f32 = src as *mut f32;

    for i in (0..(nquads << 2)).step_by(8 << 2) 
    {
        unsafe {
            _mm_storeu_ps(fdst.add(i),      _mm_loadu_ps(fsrc.add(i)));
            _mm_storeu_ps(fdst.add(i + 4),  _mm_loadu_ps(fsrc.add(i + 4)));
            _mm_storeu_ps(fdst.add(i + 8),  _mm_loadu_ps(fsrc.add(i + 8)));
            _mm_storeu_ps(fdst.add(i + 12), _mm_loadu_ps(fsrc.add(i + 12)));
            _mm_storeu_ps(fdst.add(i + 16), _mm_loadu_ps(fsrc.add(i + 16)));
            _mm_storeu_ps(fdst.add(i + 20), _mm_loadu_ps(fsrc.add(i + 20)));
            _mm_storeu_ps(fdst.add(i + 24), _mm_loadu_ps(fsrc.add(i + 24)));
            _mm_storeu_ps(fdst.add(i + 28), _mm_loadu_ps(fsrc.add(i + 28)));
        }
    }
}
