ix!();

use crate::SSOParam;

impl crate::SurgeSuperOscillator<'sr> {
    #[inline] pub fn get_t(&self, sync: f64, detune: f64) -> (f32, f32) {
        let t: f32 = {
            if self.params[SSOParam::UniSpread].absolute {

                // Oh so this line of code. What is it doing?
                //
                //  t = storage->n2pinv_tuningctr(detune * pitchmult_inv * (1.f / CONCERT_A_HZ) + sync);
                // Lets for a moment assume std tuning. So n2pinv will give you, say, 1/32 for note 60 and 1/1 for note 0. Cool.
                // it is the inverse of frequency. That's why below with detune = +/- 1 for the extreme 2 voice case we just use it directly.
                // It is the time distance of one note.
                //
                // But in absolute mode we want to scale that note. So the calculation here (assume sync is 0 for a second) is 
                // detune * pitcmult_inv / CONCERT_A_HZ
                // pitchmult_inv =  dsamplerate_os / 8.17 * n2pinv(pitch)
                // so this is using
                // detune * 1.0 / CONCERT_A_HZ * 1.0 / 8.17 * dsamplerate * n2pinv(pitch)
                // Or: 
                // detune / n2p(pitch) * ( 1.0 / (CONCERT_A_HZ * 8.17 ) ) * dsamplerate
                //
                // So there's a couple of things wrong with that. First of all this should not be samplerate dependent.
                // Second of all, what/s up with 1.0 / ( 8.17 * CONCERT_A_HZ )
                // 
                // Well the answer is that we want the time to be pushed around in hz. So it turns out that
                // 44100 * 2 / ( CONCERT_A_HZ * 8.175 ) =~ 24.2 and 24.2 / 16 = 1.447 which is almost how much absolute is off. So
                // lets set the multiplier here so that the regtests exacty match the display frequency. That is the
                // frequency desired spread / 0.9443. 0.9443 is empirically determined by running the 2 unisoncase
                // over a bunch of tests.
                let note: f64 = 
                    detune as f64 * 
                    self.tuner.n2pinv::<true,f64>( self.pitch as f64) * 
                    16.0 / 0.9443 + sync;

                let mut t = self.tuner.n2pinv::<true,f64>(note);

                // With extended range and low frequencies we can have an implied 
                // negative frequency; cut that off by setting a lower bound here.
                if t < 0.1 { 
                    t = 0.0;
                }
                t as f32
            }
            else{
                self.tuner.n2pinv_tuningctr((detune as f64) + sync) as f32
            }
        };

        let t_inv:  f32 = rcp(t);

        (t, t_inv)
    }
}
