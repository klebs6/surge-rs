crate::ix!();

/// `V_CC` is a constant voltage level used as a reference voltage; 
pub const V_CC: f32 = 1.5;

impl CheckIsAnalog for AdsrEnvelope {

    fn is_analog(&self) -> bool {
        pvalb![self.params[AdsrParam::Mode]]
    }
}
