crate::ix!();

enhanced_enum!{
    SuperOscillatorStateMachine {
        ImpulseZero,
        ImpulseOne,
        ImpulseTwo,
        ImpulseThree,
    }
}

impl SurgeSuperOscillator {

    /**
      | Delay is the number of samples ahead of
      | bufpos that oscstate implies at current
      | pitch.
      |
      | Basically the 'integer part' of the
      | position.
      */
    #[inline] pub fn get_delay(&self, 
        fm:   bool, 
        ipos: u32) -> u32 
    {
        match fm {
            true  => self.fm_delay as u32,
            false => ((ipos >> 24) & 0x3f),
        }
    }

    #[inline] fn impulse_zero(&mut self, 
        voice:  usize, 
        wf:     f32, 
        sub:    f32) -> f32
    {
        self.pwidth[voice]  = self.l_pw.v;
        self.pwidth2[voice] = 2.0 * self.l_pw2.v;

        // calculate the height of the
        // first impulse of the cycle
        let tg: f32 = {

            let b0 = 1.0 + wf;
            let b1 = 1.0 - self.pwidth[voice];
            let b2 = b0 * 0.5 + b1 * (-wf);
            let b3 = b2 * (1.0 - sub);

            let b4 = 2.0 - self.pwidth2[voice];
            let b5 = 0.5 * sub * b4;

            b3 + b5
        };

        let g: f32 = tg - self.last_level[voice];

        self.last_level[voice] = tg;

        // calculate the level the
        // sub-cycle will have at the end
        // of it's duration taking DC into
        // account
        if !CONVOLVE_NODC {

            self.last_level[voice] -= {

                let b0 = self.pwidth[voice];
                let b1 = self.pwidth2[voice];
                let b2 = 1.0 + wf;
                let b3 = 1.0 - sub;

                b0 * b1 * b2 * b3 
            }; 
        }

        g
    }

    #[inline] fn impulse_one(&mut self, 
        voice:  usize, 
        wf:     f32, 
        sub:    f32) -> f32
    {
        let g: f32 = wf * (1.0 - sub) - sub;

        self.last_level[voice] += g;

        if !CONVOLVE_NODC {

            self.last_level[voice] -= {

                let b0 = 1.0 - self.pwidth[voice];
                let b1 = 2.0 - self.pwidth2[voice];
                let b2 = 1.0 + wf;
                let b3 = 1.0 - sub;

                b0 * b1 * b2 * b3 
            };
        }

        g
    }

    #[inline] fn impulse_two(&mut self, 
        voice:  usize, 
        wf:     f32, 
        sub:    f32) -> f32
    {
        let g: f32 = 1.0 - sub;

        self.last_level[voice] += g;

        if !CONVOLVE_NODC {

            self.last_level[voice] -= {

                let b0 = self.pwidth[voice];
                let b1 = 2.0 - self.pwidth2[voice];
                let b2 = 1.0 + wf;
                let b3 = 1.0 - sub;

                b0 * b1 * b2 * b3 
            };
        }

        g
    }

    #[inline] fn impulse_three(&mut self, 
        voice:  usize, 
        wf:     f32, 
        sub:    f32) -> f32
    {
        let g: f32 = wf * (1.0 - sub) + sub;

        self.last_level[voice] += g;

        if !CONVOLVE_NODC {

            self.last_level[voice] -= {

                let b0 = 1.0 - self.pwidth[voice];
                let b1 = self.pwidth2[voice];
                let b2 = 1.0 + wf;
                let b3 = 1.0 - sub;

                b0 * b1 * b2 * b3 
            };
        }

        g
    }

    #[inline] pub fn get_g(&mut self, 
        voice:  usize, 
        wf:     f32, 
        sub:    f32, 
        stereo: bool) -> (f32, f32) {

        let state 
        = SuperOscillatorStateMachine::try_from(self.blitter.state[voice] as usize)
            .unwrap();

        /*
         | This is the super-oscillator state
         | machine; basically a 4 impulse cycle to
         | generate squares, saws, and subs.
         |
         | The output of this is 'g' which is the
         | change from the prior level at this
         | impulse.
         |
         | Each time we convolve we advance the
         | state pointer and move to the next
         | case.
         */
        let mut g: f32 = match state {
            SuperOscillatorStateMachine::ImpulseZero  => self.impulse_zero(voice, wf, sub),
            SuperOscillatorStateMachine::ImpulseOne   => self.impulse_one(voice, wf, sub),
            SuperOscillatorStateMachine::ImpulseTwo   => self.impulse_two(voice, wf, sub),
            SuperOscillatorStateMachine::ImpulseThree => self.impulse_three(voice, wf, sub),
        };

        g *= self.blitter.out_attenuation;

        let mut g_r: f32 = 0.0;

        if stereo {
            g_r = g * self.blitter.pan_r[voice];
            g *= self.blitter.pan_l[voice];
        }

        (g, g_r)
    }
}
