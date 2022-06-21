crate::ix!();

//TODO: major refactor this

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct WetBlockULS<const X: usize> {
    //lower, upper, sub
    pub l: Align16<[f32; X]>,
    pub u: Align16<[f32; X]>,
    pub s: Align16<[f32; X]>,
}

impl<const X: usize> Default for WetBlockULS<X> {
    fn default() -> Self {
        Self {
            l: Align16([0.0; X]),
            u: Align16([0.0; X]),
            s: Align16([0.0; X]),
        }
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct WetBlock1Dual<const X: usize> {
    pub buf: Align16<[[f32; X]; 2]>,
}

impl<const X: usize> Default for WetBlock1Dual<X> {
    fn default() -> Self {
        Self {
            buf: Align16([[0.0; X]; 2]),
        }
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct WetBlock1<const X: usize> {
    pub buf: Align16<[f32; X]>,
}

impl<const X: usize> Default for WetBlock1<X> {
    fn default() -> Self {
        Self {
            buf: Align16([0.0; X]),
        }
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct WetBlock1t<T: Default + std::marker::Copy ,const X: usize> {
    pub buf: Align16<[T; X]>,
}

impl<T: Default + std::marker::Copy,const X: usize> Default for WetBlock1t<T,X> {
    fn default() -> Self {
        Self {
            buf: Align16([T::default(); X]),
        }
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct WetBlock2<const X: usize> {
    pub l: Align16<[f32; X]>,
    pub r: Align16<[f32; X]>,
}

impl<const X: usize> Default for WetBlock2<X> {
    fn default() -> Self {
        Self {
            l: Align16([0.0; X]),
            r: Align16([0.0; X]),
        }
    }
}

impl<const X: usize> WetBlock2<X> {

    pub fn clear(&mut self) {
        for x in self.l.iter_mut() {
            *x = 0.0;
        }
        for x in self.r.iter_mut() {
            *x = 0.0;
        }
    }

    pub fn l(&mut self) -> *mut f32 { self.l.as_mut_ptr() }
    pub fn r(&mut self) -> *mut f32 { self.r.as_mut_ptr() }

    pub fn li<T>(&mut self, i: T) -> *mut f32 where T: Into<isize> { unsafe { self.l.as_mut_ptr().offset(i.into()) } }
    pub fn ri<T>(&mut self, i: T) -> *mut f32 where T: Into<isize> { unsafe { self.r.as_mut_ptr().offset(i.into()) } }

    pub fn dup_channel_to_stereo(&mut self, src: StereoChannel) {
        match src {
            StereoChannel::Left => 
                {
                    for (i,x) in self.l.iter().enumerate() {
                        self.r[i] = *x;
                    }
                },
            StereoChannel::Right => {
                for (i,x) in self.r.iter().enumerate() {
                    self.l[i] = *x;
                }
            }
        }
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct WetBlock {
    pub l: Align16<A1d::<f32>>,
    pub r: Align16<A1d::<f32>>,
}

impl WetBlock {
    pub fn new(len: usize) -> Self {
        Self {
            l: Align16(A1d::<f32>::zeros(len)),
            r: Align16(A1d::<f32>::zeros(len)),
        }
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct MSBlock {
    pub m: Align16<A1d::<f32>>,
    pub s: Align16<A1d::<f32>>,
}

impl MSBlock {

    pub fn new(len: usize) -> Self {
        Self {
            m: Align16(A1d::<f32>::zeros(len)),
            s: Align16(A1d::<f32>::zeros(len)),
        }
    }

    pub fn m(&mut self) -> *mut f32 {
        self.m.as_mut_ptr()
    }

    pub fn s(&mut self) -> *mut f32 {
        self.s.as_mut_ptr()
    }
}

pub type ScratchChannel<T> = Align16<A1d::<T>>;

impl<T: Clone + Zero> ScratchChannel<T> {
    pub fn new(len: usize) -> Self {
        Align16(A1d::<T>::zeros(len))
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct TBuffer {
    pub l:  ScratchChannel::<f32>,
    pub r:  ScratchChannel::<f32>,
    pub fb: ScratchChannel::<f32>,
}

impl TBuffer {

    pub fn new(len: usize) -> Self {
        Self {
            l:  Align16(A1d::<f32>::zeros(len)),
            r:  Align16(A1d::<f32>::zeros(len)),
            fb: Align16(A1d::<f32>::zeros(len)),
        }
    }

    pub fn clear_blocks(&mut self) {
        self.l.fill(0.0);
        self.r.fill(0.0);
    }

    pub fn l(&mut self) -> *mut f32 {
        self.l.as_mut_ptr()
    }

    pub fn r(&mut self) -> *mut f32 {
        self.r.as_mut_ptr()
    }

    pub fn fb(&mut self) -> *mut f32 {
        self.fb.as_mut_ptr()
    }

    pub fn li<T>(&mut self, i: T) -> *mut f32 where T: Into<isize> { unsafe { self.l.as_mut_ptr().offset(i.into()) } }
    pub fn ri<T>(&mut self, i: T) -> *mut f32 where T: Into<isize> { unsafe { self.r.as_mut_ptr().offset(i.into()) } }
}

