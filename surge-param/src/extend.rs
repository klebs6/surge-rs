crate::ix!();

pub trait GetExtendedValue
: GetExtendRange 
+ GetControlType
{

    fn get_extended(&self, f: f32) -> f32 {

        if !self.extend_range() {
            return f;
        }

        match self.control_type() {
            ControlType::FreqShift               => 100.0 * f,
            ControlType::PitchSemi7BP            => 12.0 * f,
            ControlType::PitchSemi7BPAbsolutable => 12.0 * f,
            ControlType::DecibelExtendable       => 3.0 * f,
            ControlType::DecibelNarrowExtendable => 5.0 * f,
            ControlType::OscSpread               => 12.0 * f,
            _                                    => f,
        }
    }
}

impl<P: ParameterInterface + ?Sized> GetExtendedValue for ParamRT<P> {}
