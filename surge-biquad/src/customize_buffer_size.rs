crate::ix!();

/// Includes a method for setting the buffer size
/// before processing the audio block. 
///
pub trait CustomBufferSize {

    /// allows the user to choose the buffer size
    /// that best suits their needs
    ///
    fn set_buffer_size(&mut self, buffer_size: usize);
}

