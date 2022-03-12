ix!();

#[derive(Debug)]
pub struct ControllerModulationSource {
    pub output:   f64,
    pub target:   f64,
    pub bipolar:  bool,
    pub changed:  bool,
    pub enabled:  bool,

    //can be used to assign the controller to a parameter id 
    pub id:       i32, 
    srunit: SampleRateHandle,
}

name![ControllerModulationSource,"ControllerModulationSource"];

impl ModulationSourceControl for ControllerModulationSource {
    fn get_type(&self) -> ModSrcType {
        ModSrcType::Controller
    }

    fn get_output(&self) -> f64 {
        self.output
    }

    fn set_output(&mut self, x: f64) {
        self.output = x;
    }

    fn get_output01(&self) -> f64 {
        match self.bipolar {
            true  => 0.5 + 0.5 * self.output,
            false => self.output
        }
    }

    fn is_bipolar(&self) -> bool { 
        self.bipolar
    }

    fn set_bipolar(&mut self, b: bool) {
        self.bipolar = b;
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn enable(&mut self, v: bool) {
        self.enabled = v;
    }

    fn process_block(&mut self) {
        let b: f64 = (self.target - self.output).abs();
        let a: f64 = 0.9 * 44100.0 * self.srunit.dsamplerate_inv() * b;
        self.output = (1.0 - a) * self.output + a * self.target;
    }

    fn reset(&mut self) {
        self.target = 0.0;
        self.output = 0.0;
        self.bipolar = false;
    }
}

impl ControllerModulationSource {

    pub fn new(srunit: SampleRateHandle) -> Self {
        Self {
            target:     0.0,
            output:     0.0,
            bipolar:    false,
            changed:    true,
            id:         -1,
            srunit,
            enabled:    true,
        }
    }

   pub fn process_block_until_close(&mut self, sigma: f64) -> bool
   {
       let b: f64 = (self.target - self.output).abs();

       if b < sigma {
           self.output = self.target;
           // this interpolator has reached it's target and is 
           // no longer needed
           false

       } else {
           let a: f64 = 0.9 * 44100.0 * self.srunit.dsamplerate_inv() * b;
           self.output = (1.0 - a) * self.output + a * self.target;
           true
       }
   }

    pub fn init(&mut self, f: f64) {
        self.target  = f;
        self.output  = f;
        self.changed = true;
    }

    pub fn set_target(&mut self, f: f64) {
        self.target = f;
        self.changed = true;
    }

    pub fn set_target01(&mut self, f: f64, updatechanged: bool) {
        if self.bipolar {
            self.target = 2.0 * f - 1.0;
        } else {
            self.target = f;
        }
        if updatechanged {
            self.changed = true;
        }
    }

    pub fn get_target01(&self) -> f64 {
        match self.bipolar {
            true  => 0.5 + 0.5 * self.target,
            false => self.target,
        }
    }

    pub fn has_changed(&mut self, reset: bool) -> bool {
        match self.changed {
            true => {
                if reset {
                    self.changed = false;
                }
                true
            },
            false => false,
        }
    }
}
