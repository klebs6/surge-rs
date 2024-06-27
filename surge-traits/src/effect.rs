crate::ix!();

#[enum_dispatch]
pub trait Effect : 
Debug 
+ Initialize 
+ ProcessRingout 
+ GetReturnLevel
+ Update
+ Reset
+ Suspend { }

pub type MaybeEffect = Option<Box<dyn Effect>>;
pub type MaybeEffects = Vec<MaybeEffect>;

#[enum_dispatch]
pub trait Initialize {
    fn init(&mut self) -> Result<(),SurgeError> {
        Ok(())
    }
}

#[enum_dispatch]
pub trait Update {
    fn update(&mut self) { }
}

#[enum_dispatch]
pub trait Suspend: Initialize {
    fn suspend(&mut self) -> Result<(),SurgeError> { Ok(self.init()?) }
}

#[enum_dispatch]
pub trait Reset {
    fn reset(&mut self);
}

#[enum_dispatch]
pub trait ClearBuffers {
    fn clear_buffers(&mut self) -> Result<(),SurgeError>;
}

#[enum_dispatch]
pub trait GetReturnLevel {
    fn returnlevel(&self) -> f32;
}

