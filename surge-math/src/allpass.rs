crate::ix!();

/// An all-pass filter that passes all frequencies equally
/// in gain while altering the phase relationship among
/// various frequencies.
///
/// The AllpassFilter is a signal processing filter that has
/// a constant magnitude response across the entire
/// frequency spectrum. It is commonly used in applications
/// like artificial reverberation and filter banks, where it
/// is important to maintain the overall frequency content
/// of the input signal while modifying its phase
/// characteristics.
///
/// # Parameters
///
/// * `buffer`: An array of fixed size `N` that stores the
///   filter's internal state. The size of the buffer
///   affects the filter's characteristics, including its
///   phase response and memory requirements.
///
/// * `a`: The filter coefficient that controls the amount
///   of phase shifting introduced by the filter. This value
///   should be in the range -1.0 to 1.0.
///
/// * `wpos`: The write position within the buffer, used to
///   implement a circular buffer for the filter's internal
///   state.
///
/// # Example
///
/// ```ignore
/// use crate::AllpassFilter;
///
/// let mut allpass = AllpassFilter::<8>::default();
/// let input: f64 = 0.5;
/// let output: f64 = allpass.process(input);
/// ```
///
/// # Notes
///
/// The `AllpassFilter` uses a fixed-size buffer determined
/// at compile time using const generics. This design choice
/// allows for greater efficiency and flexibility in
/// choosing the buffer size but may require recompilation
/// when changing the buffer size.
///
#[derive(Debug)]
pub struct AllpassFilter<const N: usize> {
    buffer: [f64; N],
    a:      f64,
    wpos:   usize,
}

impl<const N: usize> Default for AllpassFilter<N> {

    fn default() -> Self {
        Self {
            wpos:      0,
            a:       0.3,
            buffer: [0.0; N],
        }
    }
}

impl<const N: usize> AllpassFilter<N> {

    /// Update the write position for the buffer
    ///
    pub fn update_wpos(&mut self) {
        self.wpos = (self.wpos + 1) % N;
    }


    /// Process a sample through the AllpassFilter
    /// 
    pub fn process(&mut self, x: f64) -> f64 {

        self.update_wpos();

        let y: f64 = self.buffer[self.wpos];

        self.buffer[self.wpos] = y * -self.a + x;

        y + self.buffer[self.wpos] * self.a
    }

    pub fn get_a(&self) -> f64 {
        self.a
    }

    /// Set the filter coefficient 'a'
    ///
    pub fn set_a(&mut self, a: f64) {
        self.a = a;
    }

    pub fn get_wpos(&self) -> usize {
        self.wpos
    }

    pub fn set_wpos(&mut self, x: usize) {
        self.wpos = x;
    }

    pub fn get_buffer(&self) -> [f64;N] {
        self.buffer
    }

    pub fn get_buffer_at(&self, pos: usize) -> f64 {
        self.buffer[pos]
    }
}
