pub trait MyFloat = 
num_traits::Float 
+ std::ops::AddAssign 
+ Into<f64>;

pub type A3d<T> = ndarray::Array3::<T>;
pub type A2d<T> = ndarray::Array2::<T>;
pub type A1d<T> = ndarray::Array1::<T>;

