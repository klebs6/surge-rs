ix!();

use crate::ModSource;


#[derive(Debug)]
pub struct ModulationRouting {
    pub src:    ModSource,
    pub dst:    & mut dyn surge_param::GetSetModulation,
    pub depth:  f64,
}

