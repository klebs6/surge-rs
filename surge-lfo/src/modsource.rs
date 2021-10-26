ix!();

use crate::{
    Lfo,
};

impl ModulationSourceControl for Lfo {

    fn get_type(&self) -> ModSrcType {
        ModSrcType::LFO
    }

    fn set_output(&mut self, x: f64) {
        self.output = x;
    }

    fn get_output(&self) -> f64 {
        //TODO
        0.0
    }

    fn get_output01(&self) -> f64 {
        //TODO
        0.0
    }

    fn per_voice(&self) -> bool { 
        //TODO
        false 
    }

    fn is_bipolar(&self) -> bool { 
        true
    }

    fn set_bipolar(&mut self, b: bool) {
        assert!(b);
        /* no-op */
    }

    fn process_block(&mut self) {
        self.process();
    }

    fn enable(&mut self, v: bool) {
        self.enabled = v;
    }

    fn enabled(&self) -> bool {
        self.enabled
    }
}
