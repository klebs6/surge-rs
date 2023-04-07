crate::ix!();

pub trait AssocParam {
    type ParamType;
}

pub trait Param: Debug { 
    //___________________________________________
    fn control_type(&self)
        -> ControlType { ControlType::Nil }

    fn control_group(&self)
        -> ControlGroup { ControlGroup::Nil }

    fn control_style(&self)
        -> ControlStyle { ControlStyle::OFF }

    fn default_value(&self)
        -> PData { PData::Float(0.5) }

    fn modulateable(&self)
        -> bool { false }

    fn min_value(&self)
        -> PData { PData::Float(0.0) }

    fn max_value(&self)
        -> PData { PData::Float(1.0) }

    fn value_type(&self)
        -> ValType { ValType::VtFloat }

    fn moverate(&self)
        -> f32 { 1.0 }

    fn snap(&self)
        -> bool { true }

    fn extend_range(&self)
        -> bool { false }

    fn affect_other_parameters(&self)
        -> bool { false }

    //___________________________________________
    fn can_temposync(&self) -> bool {
        matches![self.control_type(), 
            ControlType::PortaTime 
                | ControlType::LfoRate 
                | ControlType::EnvTime 
                | ControlType::EnvTimeLfoDecay ]
    }

    fn can_extend_range(&self) -> bool {
        matches![self.control_type(),
        ControlType::PitchSemi7BP 
            | ControlType::PitchSemi7BPAbsolutable 
            | ControlType::FreqShift 
            | ControlType::DecibelExtendable
            | ControlType::DecibelNarrowExtendable
            | ControlType::OscSpread]
    }

    fn can_be_absolute(&self) -> bool {
        matches![self.control_type(),
        ControlType::OscSpread 
            | ControlType::PitchSemi7BPAbsolutable]
    }

    fn can_snap(&self) -> bool {
        matches![self.control_type(),
        ControlType::CountedSetPercent]
    }

    /// used by the gui to get the position of the modulated 
    /// handle
    fn get_modulation_f01(&self, modulation: f32) -> f32 {

        if self.control_type() == ControlType::Nil 
            || self.value_type() != ValType::VtFloat {
            return 0.0;
        }

        match (self.min_value(), self.max_value()) {
            (PData::Float(min), PData::Float(max)) => {
                let v = modulation / (max - min);
                limit_range(v, -1.0, 1.0)
            },
            _ => unreachable!(),
        }
    }

    /// used by the gui to set the modulation to match the 
    /// position of the modulated handle
    fn set_modulation_f01(&self, v: f32) -> f32 {

        if self.control_type() == ControlType::Nil 
            || self.value_type() != ValType::VtFloat {
            return 0.0;
        }

        match (self.min_value(), self.max_value()) {
            (PData::Float(min), PData::Float(max)) => 
                v * (max - min),
            _ => unreachable!(),
        }
    }

    fn get_extended(&self, f: f32) -> f32 {

        if ! self.extend_range() {
            f
        }else {
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

    fn get_default_value_f01(&self) -> f32 {

        if self.control_type() == ControlType::Nil {
            return 0.0
        }

        match ( self.default_value(), self.min_value(), self.max_value() ) {

            (PData::Float(default), PData::Float(min), PData::Float(max)) => {
              (default - min) / (max - min)
            },
            (PData::Int(default), PData::Int(min), PData::Int(max)) => {
              0.005 + 0.99 * ((default - min) as f32) / ((max - min) as f32)
            },
            (PData::Bool(default), _, _) => {
                match default { true => 1.0, false => 0.0 }
            },
            _ => 0.0,
        }
    }
}

pub trait ParameterDisplay {
    fn get_display(&self,     is_external: bool, ef: f32) -> String;
    fn get_display_alt(&self, is_external: bool, ef: f32) -> String;
}

pub trait ConvertValueToFromNormalized {
    fn value2normalized(&self, value: f32) -> f32;
    fn normalized2value(&self, value: f32) -> f32;
}

pub trait ClearFlags {
    fn clear_flags(&mut self) ;
}

pub trait GetSetModulation: Debug {
    fn set_modulation_val(&mut self, val: PData);
    fn get_modulation_val(&self) -> PData;

    fn set_param_val(&mut self, val: PData);
    fn get_param_val(&self) -> PData;

    fn get_value_f01(&self) -> f32;
    fn set_value_f01(&mut self, v: f32, is_force_integer: bool);
}

pub trait BoundValue {
    fn limit_range(&mut self);
    //TODO: this implementation relies on ParameterSetUserData, which should be eliminated
    fn bound_value(&mut self, is_force_integer: bool);
}

pub trait Morph {
    fn morph(&mut self, b: &mut Self, x: f32) -> PData;
    fn morph_alt( &mut self, a: &mut Self, b: &mut Self, x: f32);
}

pub trait ParameterController {

    fn set_parameter01<P: Param + ?Sized>(&mut self,
        param: &mut P,
        value: f32,
        is_external: bool,
        is_force_integer: bool) -> bool;

    fn send_parameter_automation(&mut self, index: i64, value: f32);
    fn get_parameter01(&mut self, index: i64) -> f32;
    fn get_parameter(&mut self, index: i64) -> f32;
    fn get_parameter_display(&mut self, index: i64, text: *mut char);
    fn get_parameter_display_alt1(&mut self, index: i64, text: *mut char, x: f32);
    fn get_parameter_display_alt2(&mut self, index: i64, text: *mut char);
    fn get_parameter_name(&mut self, index: i64, text: *mut char);
    fn get_parameter_meta(&mut self, index: i64, pm: &mut crate::ParameterMeta);
    fn set_parameter_smoothed(&mut self, index: i64, value: f32);
    fn normalized_to_value(&mut self, parameter_index: i64, value: f32) -> f32;
    fn value_to_normalized(&mut self, parameter_index: i64, value: f32) -> f32;
}


