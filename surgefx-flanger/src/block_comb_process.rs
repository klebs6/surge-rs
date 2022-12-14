crate::ix!();

impl Flanger {

    pub fn process_lfos_and_delays_for_each_comb(&mut self, 
        block_idx: usize, 
        combs:     &mut WetBlock1Dual::<BLOCK_SIZE>,
        vweights:  &mut WetBlock1Dual::<FLANGER_COMBS_PER_CHANNEL>) 
    {
        for channel_idx in 0_usize..2 {

            combs.buf[channel_idx][block_idx] = 0.0;

            for i in 0..FLANGER_COMBS_PER_CHANNEL {

                if vweights.buf[channel_idx][i as usize] > 0.0 {

                    let my_delaybase = self.delaybase[[channel_idx, i]].v;
                    let my_lfoval    = self.lfoval[[channel_idx, i]].v;

                    let tap: f32 = my_delaybase * ( 1.0 + my_lfoval * self.depth.v ) + 1.0;

                    let v:   f32 = self.idels[channel_idx].value(tap);

                    combs.buf[channel_idx][block_idx] += vweights.buf[channel_idx][i as usize] * v;
                }

                self.lfoval[[ channel_idx, i ]].process();
                self.delaybase[[ channel_idx, i ]].process();
            }
        }
    }
}
