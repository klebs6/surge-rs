#![feature(in_band_lifetimes)]

use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use core::convert::TryInto;
use surge_traits::Init;
use surge_math::{A2d,Align16};
use ndarray::Axis;

use surge_constants::{
    N_INPUTS,
    BLOCK_SIZE,
    BLOCK_SIZE_OS,
};

#[derive(Debug)]
#[repr(align(16))]
pub struct SynthInput<'synth_in> {
    pub buffer:          Align16<A2d::<f32>>,
    pub audio_in:        Align16<A2d::<f32>>,
    pub audio_in_non_os: Align16<A2d::<f32>>,
    phantom:             PhantomData<&'synth_in i32>,
}

impl Init for SynthInput<'synth_in> {
    fn init(&mut self) {
        self.audio_in.fill(0.0);
    }
}

impl Default for SynthInput<'synth_in> {
    fn default() -> Self {
        Self {
            buffer:          Align16(A2d::<f32>::zeros(( BLOCK_SIZE,    N_INPUTS ))),
            audio_in:        Align16(A2d::<f32>::zeros(( BLOCK_SIZE_OS, 2 ))),
            audio_in_non_os: Align16(A2d::<f32>::zeros(( BLOCK_SIZE,    2 ))),
            phantom:         Default::default()
        }
    }
}

#[derive(Debug,Clone)] 
pub struct SynthInputHandle<'synth_in> {
    inner: Rc<RefCell<SynthInput<'synth_in>>>,
}

//TODO: check these index_axis_mut commands --> basically we just want to index 
// properly into the buffers, although if we can do what we want without handing out
// raw pointers, that is way better
impl Default for SynthInputHandle<'synth_in> {
    fn default() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SynthInput::default())),
        }
    }
}

impl SynthInputHandle<'synth_in> {
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

impl SynthInputHandle<'synth_in> {

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
