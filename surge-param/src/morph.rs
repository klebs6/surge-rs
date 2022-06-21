crate::ix!();

impl<P: Param + Clone> crate::Morph for ParamRT<P> {

    fn morph(&mut self, b: &mut Self, x: f32) -> PData {
        let fallback = {
            if x > 0.5 {
                b.val
            } else {
                self.val
            }
        };

        match (self.val, b.val, self.control_type(), b.control_type()) {

            (PData::Float(f1), PData::Float(f2), t1, t2) => {
                if t1 == t2 {
                    PData::Float((1.0 - x) * f1 + x * f2)
                } else {
                    fallback
                }
            },
            _ => {
                fallback
            },
        }
    }

    fn morph_alt( &mut self, a: &mut Self, b: &mut Self, x: f32) {

        let mut do_fallback: bool = false;

        match ( 
            a.val,
            b.val,
            a.control_type(),
            b.control_type() 
        ) {
            (PData::Float(f1), PData::Float(f2), t1, t2) => {
                if t1 == t2 {
                    *self = a.clone();
                    self.val = PData::Float((1.0 - x) * f1 + x * f2);

                } else {
                    do_fallback = true;
                }
            }
            _ => { do_fallback = true; }
        }

        if do_fallback {
            if x > 0.5 {
                *self = b.clone();

            } else {
                *self = a.clone();
            }
        }
    }
}
