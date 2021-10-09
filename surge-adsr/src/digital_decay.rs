ix!();

use crate::{
    AdsrEnvelope,
    AdsrParam,
};

impl AdsrEnvelope<'sr> {

    fn decay_shape0_bounds(&mut self, rate: f32) -> (f32, f32) {
        (self.phase - rate, self.phase + rate)
    }

    fn decay_shape1_bounds(&mut self, rate: f32) -> (f32, f32) {

        let sx: f32 = self.phase.sqrt();

        let mut l_lo = self.phase - 2.0 * sx * rate + rate * rate;
        let     l_hi = self.phase + 2.0 * sx * rate + rate * rate;

        if( pvalf![self.params[AdsrParam::Sustain]]
            < 1e-3 && self.phase < 1e-4 ) {
            // That + rate * rate in both means at low sustain ( < 1e-3 or so) 
            // you end up with lo and hi both pushing us up off sustain. 
            // Unfortunatley we ned to handle that case specially by pushing lo down
            l_lo = 0.0;
        } 

        (l_lo, l_hi)
    }

    fn decay_shape2_bounds(&mut self, rate: f32) -> (f32, f32) {

        let sx: f32 = self.phase.powf(0.3333333);

        let l_lo = self.phase 
            - 3.0 * sx * sx * rate 
            + 3.0 * sx * rate * rate 
            - rate * rate * rate;

        let l_hi = self.phase 
            + 3.0 * sx * sx * rate 
            + 3.0 * sx * rate * rate 
            + rate * rate * rate;

        (l_lo, l_hi)
    }

    pub fn process_block_digital_decay(&mut self) {

        let decay = pvalf![self.params[AdsrParam::Decay]];

        let rate: f32 = self.tables.envelope_rate_linear(decay) 
            * tsyncratio![self,Decay];

        let sustain     = pvalf![self.params[AdsrParam::Sustain]];
        let decay_shape = pvali![self.params[AdsrParam::DecayShape]];

        let (l_lo, l_hi): (f32, f32) = match decay_shape
        {
            1 => self.decay_shape1_bounds(rate),
            2 => self.decay_shape2_bounds(rate),
            _ => self.decay_shape0_bounds(rate),
        };

        self.phase = limit_range( sustain , l_lo, l_hi );
        self.output = self.phase;
    }
}
