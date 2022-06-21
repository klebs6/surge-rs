crate::ix!();

#[derive(Debug)]
pub struct OscillatorRuntime {
    pub is_wide:             bool,
    pub tblock_l:            WetBlock1::<BLOCK_SIZE_OS>,
    pub tblock_r:            WetBlock1::<BLOCK_SIZE_OS>,
    pub drift:               f32,
    pub fmdepth:             f32,
    pub noise_colour:        f32,
    pub octave:              [f32; 3],
    pub pitch:               [f32; 3],
}

impl OscillatorRuntime {

    pub fn gen_process_cfg< const DO_FM: bool, const OSC: usize>( 
        &self 
    ) -> OscillatorProcessBlockCfg {
        match DO_FM 
        {
            true => {
                OscillatorProcessBlockCfg {
                    pitch:     self.pitch[OSC],
                    drift:     self.drift,
                    stereo:    self.is_wide,
                    fm:        true,
                    fm_depth:   self.fmdepth,
                }
            },
            false => {
                OscillatorProcessBlockCfg {
                    pitch:     self.pitch[OSC],
                    drift:     self.drift,
                    stereo:    self.is_wide,
                    ..Default::default()
                }
            }
        }
    }
}
