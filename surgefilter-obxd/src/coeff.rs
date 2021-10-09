ix!();

use crate::{
    Poles,
    Obxd24dBCoeff,
    Obxd12dBCoeff,
};

/**
  |water always seeks the lowest level, which men abhor
  |everyone wants to get to the top of the tree, but if they do, the tree would collapse
  |you too might be president
  |but who wants to be put in charge with a runaway truck?
  |Lao Tzu says: the basic position is the most powerful
  |you always get underneath the opponent
  |if he goes to attack you, you either go lower than he goes, or move in a smaller circle
  |if somebody comes into your field of centrifugal force, he gets flung out, but by his own bounce
  |So therefore, the Watercourse Way, is the way of Tao
  |We are always creating trouble by doing good to other people
  |We educate the poor for their benefit, so they desire things which they cannot get
  |But our rich people are not happy
  |But the poor people of this country are, to be judged by the way they laugh
  |So, a certain amount of doing nothing, of stopping rushing around, would cool everything
  |But also, it must be remembered that passivity is the root of all action
  |To have energy, you must sleep
  |Much more important than sleep is what I showed you at the beginning:
  |Passivity of mind, mental silence
  |You can only get to that point by realizing there is nothing you can do
  |So for God's sake don't cultivate passivity in the name of progress
  |That's like playing because it is good for your work.  But then you never get to play!
  |-Alan Watts
  */
impl CoeffMake for crate::ObxdFilter<'sr> {

    fn coeff_make(&self, freq: f32, reso: f32) -> [f32; N_COEFFMAKER_COEFFS] {

        let mut coeffs = [0.0_f32; N_COEFFMAKER_COEFFS];

        let samplerate_os_inv = self.srunit.samplerate_os_inv();

        let rcrate: f32 = (44000.0 * samplerate_os_inv).sqrt();
        let   _rcor: f32 = (500.0 / 44000.0) * rcrate;

        let cutoff: f32 = std::cmp::min(
            FloatOrd(self.tuner.n2p::<false,f32>(freq + 69.0) * (MIDI_0_FREQ as f32)), 
            FloatOrd(22000.0)).0 
            * samplerate_os_inv * PI_32;

        match self.poles {
            Poles::TwoPole => {

                coeffs[Obxd12dBCoeff::G12] = cutoff.tan();
                coeffs[Obxd12dBCoeff::R12] = 1.0 - reso;
                coeffs[Obxd12dBCoeff::BandPass] = 0.0;

                match self.sub {

                    // lowpass or lowpass self-oscillation push
                    0 | 4 => coeffs[Obxd12dBCoeff::MultiMode] = 0.0,

                    // BandPass or BandPass self-oscillation push
                    1 | 5 => {
                        coeffs[Obxd12dBCoeff::MultiMode] = 0.5;
                        coeffs[Obxd12dBCoeff::BandPass] = 1.0;
                    },

                    // highpass or highpass self-oscillation push
                    2 | 6 => {
                        coeffs[Obxd12dBCoeff::MultiMode] = 1.0;
                    },

                    // notch or notch self-oscillation push
                    3 | 7 => {
                        coeffs[Obxd12dBCoeff::MultiMode] = 0.5;
                    },
                    _ => {},
                }

                coeffs[Obxd12dBCoeff::SelfOscPush] = match self.sub > 3 {
                    true  => 1.0,
                    false => 0.0,
                }; 
            },
            Poles::FourPole => {

                coeffs[Obxd24dBCoeff::Rcor24]         = (970.0 / 44000.0) * rcrate;
                coeffs[Obxd24dBCoeff::Rcor24inv]      = 1.0 / coeffs[Obxd24dBCoeff::Rcor24];
                coeffs[Obxd24dBCoeff::G24]            = cutoff.tan();
                coeffs[Obxd24dBCoeff::R24]            = 3.5 * reso;
                coeffs[Obxd24dBCoeff::PoleMix]        = 1.0 - ((self.sub as f32) / 3.0);
                coeffs[Obxd24dBCoeff::PoleMixInvInt]  = 3.0 - (self.sub as f32);
                coeffs[Obxd24dBCoeff::PoleMixScaled]  = (coeffs[Obxd24dBCoeff::PoleMix] * 3.0) - coeffs[Obxd24dBCoeff::PoleMixInvInt];
            },
        };
        coeffs
    }
}
