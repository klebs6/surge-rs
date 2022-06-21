crate::ix!();

pub struct CombFilter {
    pub subtype: FilterSubType,
    pub tuner:   TunerHandle,
    pub srunit:  SampleRateHandle,
}

impl CombFilter {

    pub fn reso_factor(&self) -> f32 {
        match self.subtype == FilterSubType::Smooth 
        { 
            true  => -1.0, 
            false => 1.0 
        }
    }

    pub fn combmix(&self) -> f32 {
        match self.subtype == FilterSubType::Rough 
        { 
            true  =>  0.0, 
            false => 0.5 
        }
    }
}
