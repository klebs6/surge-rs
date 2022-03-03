ix!();

use crate::ModSource;

#[derive(Debug)]
pub struct ModulationRouting {
    pub src:    ModSource,
    pub dst:    Rc<RefCell<dyn surge_param::GetSetModulation>>,
    pub depth:  f64,
}

