#[cfg(target_arch = "x86_64")] 
pub use core::arch::x86_64::*;

pub use std::ops::AddAssign;
pub use std::fmt::Debug;
pub use std::convert::TryInto;
pub use surge_constants::BLOCK_SIZE;
pub use surge_math::{m128_one,m128_four,z128,m128_two};
pub use num_traits::Num;
