ix!();

impl crate::RotarySpeaker<'sr> {

    pub fn do_rotaryspeaker_post_block<const N: usize>(&mut self, 
        k: usize,
        wetblock: &mut WetBlockULS::<BLOCK_SIZE>,
        tbuffer:  &TBuffer,
        data_l:   &mut [f32; N],
        data_r:   &mut [f32; N]) 
    {
        wetblock.l[k] -= wetblock.s[k];

        let bass: f32 = wetblock.s[k] + wetblock.l[k] * ((self.lf_lfo.r as f32) * 0.6 + 0.3);

        data_l[k] = self.hornamp[0].v * tbuffer.l[k] + bass;
        data_r[k] = self.hornamp[1].v * tbuffer.r[k] + bass;

        self.lf_lfo.process();
        self.hornamp[0].process();
        self.hornamp[1].process();
    }
}
