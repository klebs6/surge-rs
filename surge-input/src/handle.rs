crate::ix!();

#[derive(Debug)]
#[repr(align(16))]
pub struct SynthInput {
    pub buffer:          Align16<A2d::<f32>>,
    pub audio_in:        Align16<A2d::<f32>>,
    pub audio_in_non_os: Align16<A2d::<f32>>,
}

impl Initialize for SynthInput {
    fn init(&mut self) -> Result<(),SurgeError> {
        self.audio_in.fill(0.0);

        Ok(())
    }
}

impl Default for SynthInput {
    fn default() -> Self {
        Self {
            buffer:          Align16(A2d::<f32>::zeros(( BLOCK_SIZE,    N_INPUTS ))),
            audio_in:        Align16(A2d::<f32>::zeros(( BLOCK_SIZE_OS, 2 ))),
            audio_in_non_os: Align16(A2d::<f32>::zeros(( BLOCK_SIZE,    2 )))
        }
    }
}

/// This code defines a Rust module and contains a few structures and implementations related to
/// audio synthesis input handling.
/// 
/// The `SynthInputHandle` has several methods that allow for the mutability of its `SynthInput`
/// inner instance.
/// 
/// - `in_left`, `in_right`, `non_os_audio_in_left`, `non_os_audio_in_right`, `audio_in_left`, and
/// `audio_in_right` all return mutable pointers to the first elements of the `buffer`,
/// `audio_in_non_os`, and `audio_in` arrays in the `SynthInput` instance.
///
/// - `audio_in0_ptr` and `audio_in1_ptr` return a mutable pointer to the element at index `idx` in
/// the `audio_in` array for channels 0 and 1, respectively.
///
/// - `non_os_audio_in0_ptr` and `non_os_audio_in1_ptr` return a constant pointer to the element at
/// index `idx` in the `audio_in_non_os` array for channels 0 and 1, respectively.
///
/// - `audio_in0` and `audio_in1` return the value of the element at index `idx` in the `audio_in`
/// array for channels 0 and 1, respectively.
/// 
/// All of these methods utilize the `index_axis_mut` and `index_axis` methods of `ArrayBase`
/// (which `A2d` is an alias for). The `index_axis_mut` method returns a mutable reference to the
/// specified axis of the array, which allows for mutable access to its elements. The `index_axis`
/// method returns a constant reference to the specified axis of the array, which only allows for
/// read-only access to its elements.
/// 
/// Additionally, these methods use the `offset` method of `NonNull`, which returns a pointer to
/// the memory location offset from the original pointer by `idx` elements. This method is unsafe
/// and requires that the index is within bounds of the array. The `TryInto` trait is used to
/// convert the `IDX` type parameter to an `isize` index.
/// 
/// Finally, the `Clone` trait is implemented for `SynthInputHandle` using the `Rc` and `RefCell`
/// types, which allows for the creation of new references to the same `SynthInput` instance. This
/// trait is useful for sharing ownership of a `SynthInputHandle` instance across multiple parts of
/// a program.
/// 
#[derive(Debug,Clone)] 
pub struct SynthInputHandle {
    inner: Rc<RefCell<SynthInput>>,
}

//TODO: check these index_axis_mut commands --> basically we just want to index 
// properly into the buffers, although if we can do what we want without handing out
// raw pointers, that is way better
impl Default for SynthInputHandle {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SynthInput::default())),
        }
    }
}

impl SynthInputHandle {
    //mutators

    #[inline] pub fn in_left(&self) -> *mut f32 {
        //self.inner.borrow_mut().buffer[0].as_mut_ptr()
        self.inner.borrow_mut().buffer.index_axis_mut(Axis(0),0).as_mut_ptr()
    }

    #[inline] pub fn in_right(&self) -> *mut f32 {
        //self.inner.borrow_mut().buffer[1].as_mut_ptr()
        self.inner.borrow_mut().buffer.index_axis_mut(Axis(0),1).as_mut_ptr()
    }

    #[inline] pub fn non_os_audio_in_left(&mut self) -> *mut f32 {
        self.inner.borrow_mut().audio_in_non_os.index_axis_mut(Axis(0),0).as_mut_ptr()
    }

    #[inline] pub fn non_os_audio_in_right(&mut self) -> *mut f32 {
        self.inner.borrow_mut().audio_in_non_os.index_axis_mut(Axis(0),1).as_mut_ptr()
    }

    #[inline] pub fn audio_in_left(&mut self) -> *mut f32 {
        self.inner.borrow_mut().audio_in.index_axis_mut(Axis(0),0).as_mut_ptr()
    }

    #[inline] pub fn audio_in_right(&mut self) -> *mut f32 {
        self.inner.borrow_mut().audio_in.index_axis_mut(Axis(0),1).as_mut_ptr()
    }

    #[inline] pub fn audio_in0_ptr<IDX>(&mut self, idx: IDX) -> *mut f32 
    where 
        IDX: TryInto<isize>,
        <IDX as TryInto<isize>>::Error: Debug
    { 
        let idx: isize = idx.try_into().unwrap(); 
        unsafe { 
            self.inner.borrow_mut().audio_in.index_axis_mut(Axis(0),0).as_mut_ptr().offset(idx)
        }
    }

    #[inline] pub fn audio_in1_ptr<IDX>(&mut self, idx: IDX) -> *mut f32 
    where 
        IDX: TryInto<isize>,
        <IDX as TryInto<isize>>::Error: Debug
    { 
        let idx: isize = idx.try_into().unwrap(); 
        unsafe {
            self.inner.borrow_mut().audio_in.index_axis_mut(Axis(0),1).as_mut_ptr().offset(idx)
        }
    }
}

impl SynthInputHandle {

    #[inline] pub fn non_os_audio_in0_ptr<IDX>(&mut self, idx: IDX) -> *const f32 
    where 
        IDX: TryInto<isize>,
        <IDX as TryInto<isize>>::Error: Debug
    { 
        let idx: isize = idx.try_into().unwrap(); 
        unsafe { 
            self.inner.borrow().audio_in_non_os.index_axis(Axis(0),0).as_ptr().offset(idx)
        }
    }

    #[inline] pub fn non_os_audio_in1_ptr<IDX>(&mut self, idx: IDX) -> *const f32 
    where 
        IDX: TryInto<isize>,
        <IDX as TryInto<isize>>::Error: Debug
    { 
        let idx: isize = idx.try_into().unwrap(); 

        unsafe { 
            self.inner.borrow().audio_in_non_os.index_axis(Axis(0),1).as_ptr().offset(idx)
        }
    }

    #[inline] pub fn audio_in0<IDX>(&mut self, idx: IDX) -> f32 
    where 
        IDX: TryInto<isize>,
        <IDX as TryInto<isize>>::Error: Debug
    { 
        let idx: isize = idx.try_into().unwrap(); 

        unsafe {
            *self.inner.borrow_mut().audio_in.index_axis_mut(Axis(0),0).as_mut_ptr().offset(idx)
        }
    }

    #[inline] pub fn audio_in1<IDX>(&mut self, idx: IDX) -> f32 
    where 
        IDX: TryInto<isize>,
        <IDX as TryInto<isize>>::Error: Debug
    { 
        let idx: isize = idx.try_into().unwrap(); 

        unsafe {
            *self.inner.borrow_mut().audio_in.index_axis_mut(Axis(0),1).as_mut_ptr().offset(idx)
        }
    }
}
