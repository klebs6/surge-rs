ix!();

#[derive(Debug)]
pub struct SynthControl {

    /**
      | updated in audio thread, read from ui,
      | so have assignments be atomic
      |
      */
    pub polydisplay:               AtomicI32, 

    pub refresh_ctrl_queue:        [i32; 8],
    pub refresh_ctrl_queue_value:  [i32; 8],

    /**
      | hosts set this if there are input busses
      |
      */
    pub process_input:             bool, 

    pub halt_engine:               bool,
}

impl Default for SynthControl {
    fn default() -> Self {
        Self {
            polydisplay:               AtomicI32::new(0), 
            refresh_ctrl_queue:        [-1; 8],
            refresh_ctrl_queue_value:  [0; 8],
            process_input:             false,
            halt_engine:               false,
        }
    }
}
