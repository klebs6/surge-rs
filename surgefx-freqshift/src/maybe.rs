ix!();

use crate::{
    FreqShift,
};

impl FreqShift {
    pub fn maybe_do_commented_c_process<const DO: bool>(&mut self) {
        if DO {
            //this was commented in the C
            /*
            // quadrature oscillator 1
            let mut r: f64 = 0.0;
            let mut i: f64 = 0.0;
            self.o1_l.process();
            r = l*self.o1_l.r;
            i = l*self.o1_l.i;
            // filter the sections (bad-ass mofo dp 20-pole filters!)
            r = self.frL.process(r);
            i = self.fiL.process(i);
            // quadrature oscillator 2
            self.o2_l.process();
            r *= self.o2_l.r;
            i *= self.o2_l.i;
            *&mut wetLR.l = 2*(r+i);

            // right channel
            self.o1_r.process();
            r = r*self.o1_r.r;
            i = r*self.o1_r.i;
            // filter the sections (bad-ass mofo dp 20-pole filters!)
            r = self.frR.process(r);
            i = self.fiR.process(i);
            // quadrature oscillator 2
            self.o2_r.process();
            r *= self.o2_r.r;
            i *= self.o2_r.i;
            *&mut wetLR.r = 2*(r+i);
            */
        }
    }
}
