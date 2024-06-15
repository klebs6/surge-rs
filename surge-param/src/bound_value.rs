crate::ix!();

pub trait BoundParameterValue: LimitParameterRange {

    // TODO: this implementation relies on ParameterSetUserData, which should be
    // eliminated
    //
    fn bound_value(&mut self, is_force_integer: bool);
}

impl<P: ParameterInterface + ?Sized> BoundParameterValue for ParamRT<P> {

    fn bound_value(&mut self, force_integer: bool) {
        match self.get_value() {
            PData::Float(f) => {
                if self.get_temposync() {

                    let (mut a, mut b) = split_float(f);

                    if b < 0.0 {
                        b += 1.0;
                        a -= 1.0;
                    }

                    /* b = min(floor(b*2.f) / 2.f,floor(b*3.f) / 3.f); //was commented */
                    b = 2.0_f32.powf(b); 

                    match b {
                        _ if b > 1.41  => b = 1.5_f32.log2(),
                        _ if b > 1.167 => b = 1.333_333_4_f32.log2(),
                        _              => b = 0.0,
                    };

                    /* val.f = floor(val.f * 4.f + 0.5f) / 4.f; // was commented*/
                    self.set_value(PData::Float(a + b)); 
                }

                if force_integer {
                    self.set_value(PData::Float((f + 0.5).floor()));
                }

                if self.snap() 
                    && self.control_type() == ControlType::CountedSetPercent 
                {
                        /*
                    if let box Some(user_data) = self.user_data {
                        todo!("workaround this somehow");
                           CountedSetUserData* cs = reinterpret_cast<CountedSetUserData*>(user_data);
                           auto count = cs->getCountedSetSize();
                        // OK so now val.f is between 0 and 1. So
                        auto fraccount = val.f * count;
                        auto intcount = (int)fraccount;
                        val.f = 1.0 * intcount / count;
                    }
                        */
                }
            },
            PData::Int(i) => {
                if self.control_type() == ControlType::VocoderBandcount {
                    self.set_value(PData::Int(i - i % 4));
                }
            },
            _ => unreachable!(),
        }
        self.limit_parameter_range();
    }
}
