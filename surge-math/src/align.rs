crate::ix!();

//TODO: is this actually getting them?
extern "platform-intrinsic" {
    pub fn simd_insert<T, E>(x: T, idx: u32, y: E) -> T;
    pub fn simd_extract<T, E>(x: T, idx: u32) -> E;
}

macro_rules! impl_align_n {
    ($n:expr) => {
        paste! {

            /**
              | The data blocks processed by the
              | SIMD instructions (e.g. SSE2),
              | which must always be before any
              | other variables in the class, in
              | order to be properly aligned to 16
              | bytes.
              */
            #[derive(Debug,Clone)] 
            #[repr(align($n))]
            pub struct [<Align $n>]<T: Clone>(pub T);

            impl<T: Clone> std::ops::Deref for [<Align $n>]<T> {
                type Target = T;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

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

#[test] fn test_align() {
    #[derive(Clone,Default)]
    struct TestStruct { }
    let x0 = Align16::<TestStruct>(TestStruct{});
    let x1 = Align32::<TestStruct>(TestStruct{});
    let x2 = Align64::<TestStruct>(TestStruct{});
    let x3 = Align128::<TestStruct>(TestStruct{});
    let x4 = Align256::<TestStruct>(TestStruct{});
    assert!(std::mem::align_of_val(&x0) == 16);
    assert!(std::mem::align_of_val(&x1) == 32);
    assert!(std::mem::align_of_val(&x2) == 64);
    assert!(std::mem::align_of_val(&x3) == 128);
    assert!(std::mem::align_of_val(&x4) == 256);
}
