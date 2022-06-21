crate::ix!();

impl WTOscillator {

    #[inline] pub fn get_ipos(&mut self, fm: bool, vidx: usize) -> u32
    {
        let p24 = 1 << 24;

        let ipos: u32 = match fm {
            true => (
                (p24 as f32) * 
                (
                    self.blitter.oscstate[vidx] * 
                    self.blitter.pitchmult_inv * 
                    self.fm_mul_inv
                )
            ) as u32,
            false =>  (
                (p24 as f32) * 
                (
                    self.blitter.oscstate[vidx] * 
                    self.blitter.pitchmult_inv
                )
            ) as u32,
        };
        ipos
    }

    #[inline] pub fn get_newlevel(&mut self, vidx: usize, block_pos: f32) -> f32
    {
        let tblip_ipol: f32 = lerp(block_pos, self.last_tableipol, self.tableipol);

        let newlevel: f32 = {

            let mipidx  = self.mipmap[vidx] as usize;
            let tidx    = self.tableid as usize;
            let blitidx = self.blitter.state[vidx] as usize;

            let lerpx   = tblip_ipol;
            let lerpa   = self.wave_wavetable.data[[mipidx,tidx,blitidx]] as f32;
            let lerpb   = self.wave_wavetable.data[[mipidx,tidx + 1,blitidx]] as f32;

            let interpd = lerp(lerpx, lerpa, lerpb);

            self.distort_level( interpd )
        };
        newlevel
    }
}
