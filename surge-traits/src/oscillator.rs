crate::ix!();

pub trait Oscillator:
OscillatorProcess 
+ AllowDisplay 
+ Initialize 
+ SetPitch 
+ HandleStreamingMismatches 
+ AssignFM 
+ OscillatorParamAccess 
+ OscillatorStereoOut
+ Debug { }

#[derive(Debug)]
pub struct OscillatorProcessBlockCfg {
    pub pitch:       f32,
    pub drift:       f32,
    pub stereo:      bool,
    pub fm:          bool,
    pub fm_depth:    f32,
}

impl Default for OscillatorProcessBlockCfg {
    fn default() -> Self {
        Self {
            pitch:   0.0, //what should our default pitch be?
            drift:   0.0,
            stereo:  false,
            fm:      false,
            fm_depth: 0.0,
        }
    }
}

pub trait OscillatorProcess {

    fn process_block(&mut self, _cfg: OscillatorProcessBlockCfg) 
        -> Result<(),SurgeError> 
    {
        Ok(())
    }
}

pub trait LoadOscillatorAlgos {
    fn load_oscalgos(&mut self) 
        -> bool ;
}

pub trait OscillatorStereoOut {
    fn out_l(&mut self) -> *mut f32 ;
    fn out_r(&mut self) -> *mut f32 ;
}

pub trait OscillatorParamAccess {
    fn osc_params(&mut self, 
        name: surge_types::OscillatorParam) 
        -> &mut surge_types::OscillatorParamRT;

    fn osc_params_const(&self, 
        name: surge_types::OscillatorParam) 
        -> &surge_types::OscillatorParamRT;
}

