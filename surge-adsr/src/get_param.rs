crate::ix!();

impl GetAttackParameter for AdsrEnvelope {
    fn get_attack_parameter(&self) -> f32 {
        pvalf![self.params[AdsrParam::Attack]]
    }
}

impl GetDecayParameter for AdsrEnvelope {
    fn get_decay_parameter(&self) -> f32 {
        pvalf![self.params[AdsrParam::Decay]]
    }
}

impl GetSustainParameter for AdsrEnvelope {
    fn get_sustain_parameter(&self) -> f32 {
        pvalf![self.params[AdsrParam::Sustain]]
    }
}

impl GetReleaseParameter for AdsrEnvelope {
    fn get_release_parameter(&self) -> f32 {
        pvalf![self.params[AdsrParam::Release]]
    }
}

impl GetAdsrParameters for AdsrEnvelope {

    // `a`, `d`, `s`, and `r` are the values of the Attack,
    // Decay, Sustain, and Release parameters respectively, scaled to the
    // range [0, 1]. 
    //
    fn get_adsr(&self) -> (f32, f32, f32, f32) {
        let a self.get_attack_parameter();
        let d self.get_decay_parameter();
        let s self.get_sustain_parameter();
        let r self.get_release_parameter();
        (a,d,s,r)
    }
}
