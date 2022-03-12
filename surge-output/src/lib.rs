#![feature(in_band_lifetimes)]
#![feature(array_methods)]

use ndarray::Axis;
use core::convert::TryInto;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use surge_math::{Align16,A2d};
use surge_constants::{
    N_OUTPUTS,
};

#[derive(Debug)]
#[repr(align(16))]
pub struct SynthOutput<const N: usize> 
{
    pub buffer:      Align16<A2d::<f32>>,
    pub vu_peak:     Align16<[f32; 8]>,
    pub masterfade:  f32,
}

impl<const N: usize> 
Default for SynthOutput<N> 
{
    fn default() -> Self {
        Self {
            buffer:     Align16(A2d::<f32>::zeros((N_OUTPUTS,N))),
            vu_peak:    Align16([0.0; 8]),
            masterfade: 1.0,
        }
    }
}

#[derive(Debug,Clone)]
pub struct SynthOutputHandle<const N: usize> {
    inner: Rc<RefCell<SynthOutput<N>>>,
}

impl<const N: usize> SynthOutputHandle<N> {

    #[inline] pub fn out_l(&mut self) -> *mut f32 {
        let buffer = self.inner.borrow_mut().buffer;
        let ax     = Axis(0);
        buffer.index_axis_mut(ax,0).as_mut_ptr()
    }

    #[inline] pub fn out_r(&mut self) -> *mut f32 {
        let buffer = self.inner.borrow_mut().buffer;
        let ax     = Axis(0);
        buffer.index_axis_mut(ax,1).as_mut_ptr()
    }

    #[inline] pub fn masterfade(&self) -> f32 { 
        self.inner.borrow_mut().masterfade
    }

    #[inline] pub fn set_masterfade<T: TryInto<f32>>(&self, x: T) 
    where <T as TryInto<f32>>::Error: Debug 
    { 
        let x: f32 = x.try_into().unwrap();
        self.inner.borrow_mut().masterfade = x;
    }

    #[inline] pub fn set_vu_peak<IDX, T>(&self, idx: IDX, x: T) 
    where 
        IDX: TryInto<usize>,
        <IDX as TryInto<usize>>::Error: Debug,
        T: TryInto<f32>,
        <T as TryInto<f32>>::Error: Debug,
    { 
        let idx: usize = idx.try_into().unwrap();
        let x:     f32 = x.try_into().unwrap();
        self.inner.borrow_mut().vu_peak[idx] = x;
    }

    #[inline] pub fn get_vu_peak<IDX>(&self, idx: IDX)  -> f32
    where 
        IDX: TryInto<usize>,
        <IDX as TryInto<usize>>::Error: Debug,
    { 
        let idx: usize = idx.try_into().unwrap();
        self.inner.borrow_mut().vu_peak[idx]
    }
}
