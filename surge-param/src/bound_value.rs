ix!();

use crate::{PData,BoundValue,ControlType,Param,ParamRT};

impl<P: Param + ?Sized> BoundValue for ParamRT<P> {

    fn limit_range(&mut self) {

        /*
           |clamp value between parameter minimum
           |and maximum should not need to be
           |called from the outside, because
           |a limited range is and invariant which
           |should be upheld internally
           */
        match (self.val, self.min_value(), self.max_value()) {
            (PData::Float(f), PData::Float(min), PData::Float(max)) => {
                self.val = PData::Float(limit_range(f,min,max));
            },
            (PData::Int(i), PData::Int(min), PData::Int(max)) => {
                self.val = PData::Int(limit_range(i,min,max));
            },
            _ => { /*noop*/ }
        }
    }

    fn bound_value(&mut self, force_integer: bool) {
        match self.val {
            PData::Float(f) => {
                if self.temposync {

                    let (mut a, mut b) = split_float(f);

                    if b < 0.0 {
                        b += 1.0;
                        a -= 1.0;
                    }

                    b = 2.0_f32.powf(b); /* b = min(floor(b*2.f) / 2.f,floor(b*3.f) / 3.f); //was commented */

                    match b {
                        _ if b > 1.41  => b = 1.5_f32.log2(),
                        _ if b > 1.167 => b = 1.333_333_4_f32.log2(),
                        _              => b = 0.0,
                    };

                    self.val = PData::Float(a + b); /* val.f = floor(val.f * 4.f + 0.5f) / 4.f; // was commented*/
                }

                if force_integer {
                    self.val = PData::Float((f + 0.5).floor());
                }

                if self.snap 
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
                    self.val = PData::Int(i - i % 4);
                }
            },
            _ => unreachable!(),
        }
        self.limit_range();
    }
}
