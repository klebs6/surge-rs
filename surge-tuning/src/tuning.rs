crate::ix!();

#[derive(Debug,Getters,MutGetters,Clone)]
#[repr(align(16))]
pub struct SurgeTuning {
    #[getset(get)] pub pitch:              f32,
    #[getset(get)] pub pitch_inv:          f32,
    #[getset(get)] pub is_standard_tuning: bool,
}

impl SurgeTuning {
    pub fn new() -> Result<Self,SurgeError> {
        let  mut x = Self {
            pitch: 32.0,
            pitch_inv: 1.0 / 32.0,
            is_standard_tuning: true,
        };
        x.init()?;
        Ok(x)
    }
}

impl Initialize for SurgeTuning {
    fn init(&mut self) -> Result<(),SurgeError> {
        self.is_standard_tuning = true;

        Ok(())
    }
}

impl SetPitch for SurgeTuning {
    fn set_pitch(&mut self, x: f32, _is_display: bool) {
        self.pitch = x;
        self.is_standard_tuning = false;
    }
}
