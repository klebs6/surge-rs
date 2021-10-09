ix!();

use crate::ProcessRingout;

#[enum_dispatch]
pub trait Effect : 
Debug 
+ Init 
+ ProcessRingout 
+ GetReturnLevel
+ Update
+ Suspend { }

pub type MaybeEffect<'sr> = Option<Box<dyn Effect + 'sr>>;
pub type MaybeEffects<'sr> = Vec<MaybeEffect<'sr>>;

#[enum_dispatch]
pub trait Init {
    fn init(&mut self) {}
}

#[enum_dispatch]
pub trait Update {
    fn update(&mut self) { }
}

#[enum_dispatch]
pub trait Suspend: Init {
    fn suspend(&mut self) { self.init() }
}

#[enum_dispatch]
pub trait Reset {
    fn reset(&mut self);
}

#[enum_dispatch]
pub trait ClearBuffers {
    fn clear_buffers(&mut self);
}

#[enum_dispatch]
pub trait GetReturnLevel {
    fn returnlevel(&self) -> f32;
}

