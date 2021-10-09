ix!();

use crate::ModSource;


#[derive(Debug)]
pub struct ModulationRouting<'sr> {
    pub src:    ModSource,
    pub dst:    &'sr mut dyn surge_param::GetSetModulation,
    pub depth:  f64,
}

