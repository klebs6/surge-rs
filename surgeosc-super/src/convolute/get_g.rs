ix!();

use crate::{
    SurgeSuperOscillator,
    CONVOLVE_NODC
};

impl SurgeSuperOscillator<'sr> {

    /// Delay is the number of samples ahead of bufpos that oscstate implies at current pitch.
    /// Basically the 'integer part' of the position.
    #[inline] pub fn get_delay(&self, fm: bool, ipos: u32) -> u32 {
        match fm {
            true  => self.fm_delay as u32,
            false => ((ipos >> 24) & 0x3f),
        }
    }

    #[inline] pub fn get_g(&mut self, voice: usize, wf: f32, sub: f32, stereo: bool) -> (f32, f32) {

        let mut g:  f32;
        let mut g_r: f32 = 0.0;

        // This is the super-oscillator state machine; basically a 4 impulse cycle to generate
        // squares, saws, and subs. The output of this is 'g' which is the change from the prior
        // level at this impulse. Each time we convolve we advance the state pointer and move to the
        // next case.
        match self.blitter.state[voice] {
            0 => {

                self.pwidth[voice] = self.l_pw.v;
                self.pwidth2[voice] = 2.0 * self.l_pw2.v;

                // calculate the height of the first impulse of the cycle
                let tg: f32 = ((1.0 + wf) * 0.5 + (1.0 - self.pwidth[voice]) * (-wf)) * (1.0 - sub) + 0.5 * sub * (2.0 - self.pwidth2[voice]);

                g = tg - self.last_level[voice];
                self.last_level[voice] = tg;

                // calculate the level the sub-cycle will have at the end of it's duration taking DC into account
                if !CONVOLVE_NODC {
                    self.last_level[voice] -= (self.pwidth[voice]) * (self.pwidth2[voice]) * (1.0 + wf) * (1.0 - sub); 
                }

            },
            1 => {

                g = wf * (1.0 - sub) - sub;

                self.last_level[voice] += g;

                if !CONVOLVE_NODC {
                    self.last_level[voice] -= (1.0 - self.pwidth[voice]) * (2.0 - self.pwidth2[voice]) * (1.0 + wf) * (1.0 - sub);
                }

            },
            2 => {

                g = 1.0 - sub;

                self.last_level[voice] += g;

                if !CONVOLVE_NODC {
                    self.last_level[voice] -= (self.pwidth[voice]) * (2.0 - self.pwidth2[voice]) * (1.0 + wf) * (1.0 - sub);
                }

            },
            3 => {
                g = wf * (1.0 - sub) + sub;

                self.last_level[voice] += g;

                if !CONVOLVE_NODC {
                    self.last_level[voice] -= (1.0 - self.pwidth[voice]) * (self.pwidth2[voice]) * (1.0 + wf) * (1.0 - sub);
                }
            },
            _ => unreachable!("create an enum for this match tree")
        }

        g *= self.blitter.out_attenuation;

        if stereo {
            g_r = g * self.blitter.pan_r[voice];
            g *= self.blitter.pan_l[voice];
        }

        (g, g_r)
    }
}
