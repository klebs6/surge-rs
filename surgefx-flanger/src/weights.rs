crate::ix!();

impl Flanger {

    pub fn fill_weights(&mut self, 
        channel_idx: usize, 
        mtype:       FlangerType,
        vweights:    &mut WetBlock1Dual::<FLANGER_COMBS_PER_CHANNEL>) 
    {
        for i in 0..FLANGER_COMBS_PER_CHANNEL {
            vweights.buf[channel_idx][i] = 0.0;
        }

        match mtype {
            FlangerType::Arp => {

                let mut ilp: i32 = self.longphase as i32;

                let flp: f32 = self.longphase - (ilp as f32);

                if ilp == FLANGER_COMBS_PER_CHANNEL as i32 {
                    ilp = 0;
                }

                if flp > 0.9 {
                    let dt:  f32 = (flp - 0.9) * 10.0; // this will be between 0,1
                    let nxt: f32 = dt.sqrt();
                    let prr: f32 = ( 1.0 - dt ).sqrt();

                    vweights.buf[channel_idx][ilp as usize] = prr;

                    if ilp == (FLANGER_COMBS_PER_CHANNEL as i32 - 1) {
                        vweights.buf[channel_idx][0] = nxt;
                    } else {
                        vweights.buf[channel_idx][(ilp + 1) as usize] = nxt;
                    }

                } else {

                    vweights.buf[channel_idx][ilp as usize] = 1.0;
                }

            },
            _ => {
                let voices: i32 = self.pvali(FlangerParam::Voices);

                vweights.buf[channel_idx][0] = 1.0;

                for i in 0..voices {
                    if i == 4 {
                        break;
                    }
                    vweights.buf[channel_idx][i as usize] = 1.0;
                }

                let li: i32 = voices;
                let fi: f32 = (voices as f32) - (li as f32);

                if li < 4  {
                    vweights.buf[channel_idx][li as usize] = fi;
                }
            }
        }
    }
}
