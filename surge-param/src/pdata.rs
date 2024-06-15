crate::ix!();

#[derive(Debug,Copy, Clone)]
pub enum PData {
    Int(i32),
    Bool(bool),
    Float(f32),
}

macro_rules! impl_pdata_try_into [
    ($ty:ty, $x:ident) => {

        impl TryInto<$ty> for PData {

            type Error = core::fmt::Error;

            fn try_into(self) -> Result<$ty, Self::Error> {
                return match self {
                    PData::$x(val) => Ok(val.into()),
                    _ => panic!(),
                }
            }
        }
    }
];

impl_pdata_try_into![f64,  Float];
impl_pdata_try_into![f32,  Float];
impl_pdata_try_into![i32,  Int];
impl_pdata_try_into![bool, Bool];

//can we factor this out?
//this was necessary in C,
//but probably not in rust
enhanced_enum![
    ValType {
        VtInt,
        VtBool,
        VtFloat,
    }
];

//do we want to leave this in here since there is a panic case?
impl std::ops::Sub for PData {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (PData::Float(x1),   PData::Float(x2)) => { PData::Float(x1 - x2)        },
            (PData::Float(x1),   PData::Int(x2))   => { PData::Float(x1 - x2 as f32) },
            (PData::Int(x1),     PData::Float(x2)) => { PData::Float(x1 as f32 - x2) },
            (PData::Int(x1),     PData::Int(x2))   => { PData::Int(x1 - x2)          },
            _ => { panic!("incompatible types for ops::Sub! program logic bug!");  }
        }
    }
}
