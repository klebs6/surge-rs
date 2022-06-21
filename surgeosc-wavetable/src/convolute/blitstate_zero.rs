crate::ix!();

impl WTOscillator {

    #[inline] pub fn do_blitstate_zero_for_convolute(&mut self, vidx: usize) {

        let n_tables: usize =  self.wave_wavetable.num_tables();

        self.formant_last = self.formant_t;
        self.last_hskew   = self.hskew;
        self.hskew        = self.l_hskew.v;

        if wt_flag![self,IsSample] {

            self.tableid += 1;

            if self.tableid > (n_tables as i32) - 3 {

                if self.sampleloop < 7 {
                    self.sampleloop -= 1;
                }

                match self.sampleloop > 0 {
                    true => self.tableid = 0,
                    false => {
                        self.tableid = (n_tables as i32) - 2;

                        // rather large number
                        self.blitter.oscstate[vidx] = 100000000000.0; 
                        return;
                    },
                }
            }
        }

        let ts: usize = self.wave_wavetable.num_samples_per_table();
        let a:  f32   = self.wave_wavetable.dt() * self.blitter.pitchmult_inv;

        let wtbias: f32 = 1.80;

        self.mipmap[vidx] = match (a, ts) {
            (_,_) if (a < 0.015625 * wtbias) && (ts >= 128) => 6,
            (_,_) if (a < 0.03125  * wtbias) && (ts >= 64)  => 5,
            (_,_) if (a < 0.0625   * wtbias) && (ts >= 32)  => 4,
            (_,_) if (a < 0.125    * wtbias) && (ts >= 16)  => 3,
            (_,_) if (a < 0.25     * wtbias) && (ts >= 8)   => 2,
            (_,_) if (a < 0.5      * wtbias) && (ts >= 4)   => 1,
            (_,_)                                           => 0,
        };

        // wt_inc = (1<<self.mipmap[i]);
        self.mipmap_ofs[vidx] = 0;

        for i in 0..(self.mipmap[vidx]) {
            self.mipmap_ofs[vidx] += (ts >> i) as i32;
        }
    }
}
