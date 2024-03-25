crate::ix!();

// Declare the external SIMD functions for inserting and
// extracting elements
//
// TODO: Verify if the functions are correctly imported
//
/* extern "platform-intrinsic" { } */
pub fn simd_insert<T, E>(_x: T, _idx: u32, _y: E) -> T {
    todo!();
}

pub fn simd_extract<T, E>(_x: T, _idx: u32) -> E {
    todo!();
}

macro_rules! impl_align_n {
    ($n:expr) => {
        paste! {

            /// Define a struct with the specified alignment
            ///
            /// The struct wraps a type T and ensures that the data is aligned
            /// to the specified number of bytes. This is required for some
            /// SIMD instructions that expect data to be properly aligned.
            ///
            /// # Example
            ///
            /// ```
            /// #[derive(Clone, Default)]
            /// struct TestStruct { }
            ///
            /// let x = Align16::<TestStruct>(TestStruct{});
            /// ```
            #[derive(Debug,Clone)] 
            #[repr(align($n))]
            pub struct [<Align $n>]<T: Clone>(pub T);

             /// Implement the Deref trait for the generated struct
             ///
            impl<T: Clone> std::ops::Deref for [<Align $n>]<T> {
                type Target = T;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

             /// Implement the DerefMut trait for the generated struct
             ///
            impl<T: Clone> std::ops::DerefMut for [<Align $n>]<T> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        }
    }
}

impl_align_n![16];
impl_align_n![32];
impl_align_n![64];
impl_align_n![128];
impl_align_n![256];

#[macro_export] macro_rules! align16  { ($data:expr) => { $crate::Align16($data) } }
#[macro_export] macro_rules! align32  { ($data:expr) => { $crate::Align32($data) } }
#[macro_export] macro_rules! align64  { ($data:expr) => { $crate::Align64($data) } }
#[macro_export] macro_rules! align128 { ($data:expr) => { $crate::Align128($data) } }
#[macro_export] macro_rules! align256 { ($data:expr) => { $crate::Align256($data) } }
