crate::ix!();

impl Flanger {

    pub fn init_comb(&mut self, 
        ty:           FlangerType, 
        wave:         FlangerWave, 
        v0pitch:      f32, 
        rate:         f32, 
        channel_idx:  usize, 
        comb_idx:     usize) 
    {
        self.lfophase[[channel_idx,comb_idx]] += rate;

        let mut lforeset: bool = false;

        if self.lfophase[[channel_idx,comb_idx]] > 1.0 
        {
            lforeset = true;
            self.lfophase[[channel_idx,comb_idx]] -= 1.0;
        }

        let mut lfoout:    f32 = self.lfoval[[channel_idx,comb_idx]].v;
        let mut thisphase: f32 = self.lfophase[[channel_idx,comb_idx]];

        if ty == FlangerType::Arp 
        {
            // arpeggio - everyone needs to use the same phase with the voice swap
            thisphase = self.longphase - ((self.longphase as i32) as f32);
        }

        match wave {

            FlangerWave::Tri => {
                lfoout = 2.0 * (2.0 * thisphase - 1.0).abs() - 1.0;
            },

            FlangerWave::Sin => {
                let ps: f32 = thisphase * (FLANGER_LFO_TABLE_SIZE as f32);
                let psi: i32 = ps as i32;
                let psf: f32 = ps - (psi as f32);
                let psn: i32 = ( psi + 1 ) & (FLANGER_LFO_TABLE_MASK as i32);
                lfoout = self.sin_lfo_table[psi as usize] * ( 1.0 - psf ) + 
                    psf * self.sin_lfo_table[psn as usize];
            },

            FlangerWave::Saw => {
                //we gotta be gentler than a pure saw. So FIXME on this waveform
                lfoout = thisphase * 2.0 - 1.0;
            },

            FlangerWave::SampleAndHold => {
                // S&h random noise. Needs smoothing over the jump like the triangle
                if lforeset {
                    lfoout = rand01();
                }
            },
        }

        let pitchinv: f32 = self.tuner.n2pinv::<f32,false>(v0pitch + (comb_idx as f32) * 6.0);

        self.lfoval[[channel_idx,comb_idx]].new_value(lfoout);
        self.delaybase[[channel_idx,comb_idx]].new_value( 
            self.srunit.samplerate() * 
            (FLANGER_ONE_OVER_F0 as f32) *  
            pitchinv);
    }
}
