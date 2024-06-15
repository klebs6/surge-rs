crate::ix!();

pub trait GetExtendRange {

    fn extend_range(&self)
        -> bool { false }
}

impl<P: ParameterInterface + ?Sized> GetExtendRange for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn extend_range(&self) -> bool;
        }
    }
}

pub trait CheckIfCanExtendRange: GetControlType {

    fn can_extend_range(&self) -> bool {
        matches!{
            self.control_type(),
            ControlType::PitchSemi7BP 
                | ControlType::PitchSemi7BPAbsolutable 
                | ControlType::FreqShift 
                | ControlType::DecibelExtendable
                | ControlType::DecibelNarrowExtendable
                | ControlType::OscSpread
        }
    }
}

impl<P: ParameterInterface + ?Sized> CheckIfCanExtendRange for ParamRT<P> {}
