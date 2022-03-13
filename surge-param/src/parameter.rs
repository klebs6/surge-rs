ix!();

use crate::{Param,PData,ControlType,ControlStyle,ControlGroup,ValType,BoundValue};

pub type ControlGroupEntry = i16;

#[derive(Debug,Clone)]
pub struct ParamRT<P: Param + ?Sized> {
    pub val:                    PData,//want interior mutability
    pub modulation_delta:       PData,//want interior mutability

    pub midictrl:               Option<i32>,

    //are these compile time values?
    pub per_voice_processing:   bool,
    pub temposync:              bool,
    pub extend_range:           bool,
    pub absolute:               bool,
    pub snap:                   bool,
    pub delegate:               Box<P>,
}

impl<P: Param + ?Sized> Param for ParamRT<P> {

    fn control_type(&self)
        -> ControlType { self.delegate.control_type() }

    fn control_style(&self)
        -> ControlStyle { self.delegate.control_style() }

    fn control_group(&self)
        -> ControlGroup { self.delegate.control_group() }

    fn default_value(&self)
        -> PData { self.delegate.default_value() }

    fn modulateable(&self)
        -> bool { self.delegate.modulateable() }

    fn min_value(&self)
        -> PData { self.delegate.min_value() }

    fn max_value(&self)
        -> PData { self.delegate.max_value() }

    fn value_type(&self)
        -> ValType { self.delegate.value_type() }

    fn moverate(&self)
        -> f32 { self.delegate.moverate() }

    fn snap(&self)
        -> bool { self.delegate.snap() }

    fn extend_range(&self)
        -> bool { self.delegate.extend_range() }

    fn affect_other_parameters(&self)
        -> bool { self.delegate.affect_other_parameters() }
}

impl<P: Param> ParamRT<P> {

    pub fn default_modulation_delta(default_val: PData) -> PData {
        match default_val {
            PData::Float(_x) => PData::Float(0.0),
            PData::Int(_x)   => PData::Int(0),
            PData::Bool(_x)  => PData::Bool(false),
        }
    }

    pub fn new( delegate: P ) -> Self {

        let default_val = delegate.default_value();

        let mut x = Self {
            delegate:             Box::new(delegate),
            val:                  default_val,
            modulation_delta:     Self::default_modulation_delta(default_val),
            midictrl:             None,
            per_voice_processing: true,
            temposync:            false,
            extend_range:         false,
            absolute:             false,
            snap:                 false,
        };
        let force_integer = false;
        x.bound_value(force_integer);
        x
    }
}
