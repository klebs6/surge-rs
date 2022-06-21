crate::ix!();

impl RotarySpeaker {

    pub fn do_rotaryspeaker_block(&mut self, 
        k:        usize, 
        tbuffer:  &mut TBuffer,
        wetblock: &mut WetBlockULS::<BLOCK_SIZE>) 
    {
        // feed delay input
        let wp: i32 = (self.wpos + k as i32) & ((ROTARY_SPEAKER_MAX_DELAY_LENGTH - 1) as i32);

        wetblock.s[k] = wetblock.l[k];
        wetblock.u[k] -= wetblock.l[k];
        self.buffer[wp as usize] = wetblock.u[k];

        let i_dtime_l: i32 = maxi(BLOCK_SIZE as i32, mini(self.d_l.v as i32, (ROTARY_SPEAKER_MAX_DELAY_LENGTH - FIR_IPOL_N - 1) as i32));
        let i_dtime_r: i32 = maxi(BLOCK_SIZE as i32, mini(self.d_r.v as i32, (ROTARY_SPEAKER_MAX_DELAY_LENGTH - FIR_IPOL_N - 1) as i32));

        let rp_l: i32 = self.wpos - i_dtime_l + k as i32;
        let rp_r:  i32 = self.wpos - i_dtime_r + k as i32;

        let sinc_l: i32 = (
            (FIR_IPOL_N as f32) *
            limit_range(
                (FIR_IPOL_M as f32) * (((i_dtime_l + 1) as f32) - self.d_l.v), 
                0.0, 
                (FIR_IPOL_M - 1) as f32
            )
        ) as i32;

        let sinc_r: i32 = ((FIR_IPOL_N as f32) *
            limit_range((FIR_IPOL_M as f32) * (((i_dtime_r + 1) as f32) - self.d_r.v), 0.0, (FIR_IPOL_M - 1) as f32)) as i32;

        // get delay output
        tbuffer.l[k] = 0.0;
        tbuffer.r[k] = 0.0;

        for i in 0..FIR_IPOL_N {

            let idx_l: usize = ((rp_l - i as i32) & (ROTARY_SPEAKER_MAX_DELAY_LENGTH as i32 - 1)).try_into().unwrap();
            let idx_r: usize = ((rp_r - i as i32) & (ROTARY_SPEAKER_MAX_DELAY_LENGTH as i32 - 1)).try_into().unwrap();

            let sincidx_l: usize = (sinc_l + FIR_IPOL_N as i32 - i as i32).try_into().unwrap();
            let sincidx_r: usize = (sinc_r + FIR_IPOL_N as i32 - i as i32).try_into().unwrap();

            tbuffer.l[k] += self.buffer[idx_l] * self.tables.sinctable_1x(sincidx_l);
            tbuffer.r[k] += self.buffer[idx_r] * self.tables.sinctable_1x(sincidx_r);
        }

        self.d_l.process();
        self.d_r.process();
    }
}
