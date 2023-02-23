/// Here are some examples of how you could
/// improve the interface for these traits:
/// 
/// 1. **Use Rust slices instead of raw
/// pointers.** Rather than using raw pointers for
/// the data inputs and outputs, you could use
/// Rust slices. Slices are a safe and ergonomic
/// way to pass around arrays in Rust, and would
/// make it easier for users to work with the
/// audio data.
/// 
/// 2. **Add more safety guarantees.** You could
/// add more safety guarantees to the functions by
/// specifying the size of the data inputs and
/// outputs, and ensuring that the input and
/// output slices have the same length. You could
/// also add documentation specifying any other
/// safety requirements, such as ensuring that the
/// input slices are not null.
///
/// With these changes, users could call the
/// functions like this:
/// 
/// ```
/// let mut processor = MyProcessor::new();
/// let mut input = vec![0.0; BLOCK_SIZE];
/// let mut output = vec![0.0; BLOCK_SIZE];
/// processor.process_mono_block(&input, Some(&mut output));

crate::ix!();

/// Trait for processing a block of mono audio
/// samples.
///
pub trait ProcessBlockMono {

     /// # Safety
     ///
     /// `data` must point to `BLOCK_SIZE` valid
     /// contiguous data elements.
     ///
     /// If `out` is not `None`, `out.unwrap()`
     /// must also point to `BLOCK_SIZE` valid
     /// contiguous data elements.
     ///
    unsafe fn process_block_mono(
        &mut self, 
        data: *mut f32, 
        out: Option<*mut f32>);

    fn process_mono_block(
        &mut self,
        input: &[f32],
        output: Option<&mut [f32]>
    );
}

/// Trait for processing a block of stereo audio
/// samples.
///
pub trait ProcessBlockStereo {

    /// # Safety
    ///
    /// `data_l` and `data_r` must each point to
    /// `BLOCK_SIZE` valid contiguous data
    /// elements.
    ///
    /// If `out` is not `None`, `out.unwrap()`
    /// must be a tuple of two pointers each
    /// pointing to `BLOCK_SIZE` valid contiguous
    /// data elements.
    ///
    unsafe fn process_block_stereo(
        &mut self, 
        data_l: *mut f32, 
        data_r: *mut f32, 
        out: Option<(*mut f32, *mut f32)>
    );

    fn process_stereo_block(
        &mut self,
        input_left: &[f32],
        input_right: &[f32],
        output_left: Option<&mut [f32]>,
        output_right: Option<&mut [f32]>
    );
}

/// Trait for processing a block of stereo audio
/// samples with a slower lag.
///
pub trait ProcessBlockSlowlag {

    /// # Safety
    ///
    /// `data_l` and `data_r` must each point to
    /// `BLOCK_SIZE` valid contiguous data
    /// elements.
    ///
    unsafe fn process_block_slowlag(
        &mut self, 
        data_l: *mut f32, 
        data_r: *mut f32);

    fn process_slowlag_block(
        &mut self,
        input_left: &[f32],
        input_right: &[f32]
    );
}

/// Includes a method for setting the buffer size
/// before processing the audio block. 
///
pub trait CustomBufferSize {

    /// allows the user to choose the buffer size
    /// that best suits their needs
    ///
    fn set_buffer_size(&mut self, buffer_size: usize);
}
