/// hello! could you please help me document the following code? high detail is appreciated!
/// 
/// This code defines a Rust module and contains a few structures and implementations related to audio synthesis input handling.
/// 
/// ```
/// crate::ix!();
/// 
/// #[derive(Debug)]
/// #[repr(align(16))]
/// pub struct SynthInput {
///     pub buffer:          Align16<A2d::<f32>>,
///     pub audio_in:        Align16<A2d::<f32>>,
///     pub audio_in_non_os: Align16<A2d::<f32>>,
/// }
/// ```
/// 
/// - `ix!();` is a Rust macro that is used to define an index space for use with the `ndarray` crate.
/// - `#[derive(Debug)]` is an attribute macro that implements the `Debug` trait for the struct.
/// - `#[repr(align(16))]` is an attribute macro that specifies a memory alignment of 16 bytes for the struct.
/// - `SynthInput` is a struct that contains three fields:
///   - `buffer`: an `Align16` wrapper around an `A2d` (2-dimensional `ndarray`) of 32-bit floating-point numbers (`f32`).
///   - `audio_in`: an `Align16` wrapper around an `A2d` of `f32` values.
///   - `audio_in_non_os`: an `Align16` wrapper around an `A2d` of `f32` values.
/// 
/// ```
/// impl Init for SynthInput {
///     fn init(&mut self) {
///         self.audio_in.fill(0.0);
///     }
/// }
/// ```
/// 
/// - `Init` is a custom trait that defines an `init` method.
/// - `impl Init for SynthInput` means that this implementation of `init` applies only to instances of `SynthInput`.
/// - `fn init(&mut self)` is the implementation of the `init` method.
/// - `self.audio_in.fill(0.0)` fills the `audio_in` field with `0.0` values.
/// 
/// ```
/// impl Default for SynthInput {
///     fn default() -> Self {
///         Self {
///             buffer:          Align16(A2d::<f32>::zeros(( BLOCK_SIZE,    N_INPUTS ))),
///             audio_in:        Align16(A2d::<f32>::zeros(( BLOCK_SIZE_OS, 2 ))),
///             audio_in_non_os: Align16(A2d::<f32>::zeros(( BLOCK_SIZE,    2 )))
///         }
///     }
/// }
/// ```
/// 
/// - `Default` is a built-in trait that defines a default value for a type.
/// - `impl Default for SynthInput` means that this implementation of `Default` applies only to instances of `SynthInput`.
/// - `fn default() -> Self` is the implementation of the `default` method.
/// - `Self` refers to the type of the struct, `SynthInput`.
/// - `Align16` is a wrapper type that ensures memory alignment for its contained value.
/// - `A2d` is a type alias for a 2-dimensional `ndarray`.
/// - `zeros` is a method of `ndarray` that creates an array of the specified shape filled with `0.0` values.
/// - The `Self` struct is constructed with `Align16` wrappers around three `A2d` arrays filled with `0.0` values of the specified shape.
/// 
/// ```
/// #[derive(Debug,Clone)]
/// pub struct SynthInputHandle {
///     inner: Rc<RefCell<SynthInput>>,
/// }
/// ```
/// 
/// - `#[derive(Debug,Clone)]` specifies that the struct should implement the `Debug` and `Clone` traits.
/// - `SynthInputHandle` is a struct that contains a single field:
///   - `inner`: a reference-counted smart pointer (`Rc`) to a mutable borrow-checked (`RefCell
/// 
/// 
/// The `Default` trait is implemented for `SynthInputHandle`. This trait provides a way to get a default value for a type. In this case, the default value is an instance of `SynthInputHandle` with a reference counted `Rc` inner `SynthInput` instance that is initialized using the `default` function of `SynthInput`.
/// 
/// The `SynthInputHandle` has several methods that allow for the mutability of its `SynthInput` inner instance.
/// 
/// - `in_left`, `in_right`, `non_os_audio_in_left`, `non_os_audio_in_right`, `audio_in_left`, and `audio_in_right` all return mutable pointers to the first elements of the `buffer`, `audio_in_non_os`, and `audio_in` arrays in the `SynthInput` instance.
/// - `audio_in0_ptr` and `audio_in1_ptr` return a mutable pointer to the element at index `idx` in the `audio_in` array for channels 0 and 1, respectively.
/// - `non_os_audio_in0_ptr` and `non_os_audio_in1_ptr` return a constant pointer to the element at index `idx` in the `audio_in_non_os` array for channels 0 and 1, respectively.
/// - `audio_in0` and `audio_in1` return the value of the element at index `idx` in the `audio_in` array for channels 0 and 1, respectively.
/// 
/// All of these methods utilize the `index_axis_mut` and `index_axis` methods of `ArrayBase` (which `A2d` is an alias for). The `index_axis_mut` method returns a mutable reference to the specified axis of the array, which allows for mutable access to its elements. The `index_axis` method returns a constant reference to the specified axis of the array, which only allows for read-only access to its elements.
/// 
/// Additionally, these methods use the `offset` method of `NonNull`, which returns a pointer to the memory location offset from the original pointer by `idx` elements. This method is unsafe and requires that the index is within bounds of the array. The `TryInto` trait is used to convert the `IDX` type parameter to an `isize` index.
/// 
/// Finally, the `Clone` trait is implemented for `SynthInputHandle` using the `Rc` and `RefCell` types, which allows for the creation of new references to the same `SynthInput` instance. This trait is useful for sharing ownership of a `SynthInputHandle` instance across multiple parts of a program.
/// 


crate::ix!();

#[derive(Debug)]
#[repr(align(16))]
pub struct SynthInput {
    pub buffer:          Align16<A2d::<f32>>,
    pub audio_in:        Align16<A2d::<f32>>,
    pub audio_in_non_os: Align16<A2d::<f32>>,
}

impl Init for SynthInput {
    fn init(&mut self) {
        self.audio_in.fill(0.0);
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
