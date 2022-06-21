crate::ix!();

impl SetPitch for WTOscillator {

    fn set_pitch(&mut self, pitch: f32, is_display: bool) { 

        if is_display {
            self.blitter.n_unison = 1;
        }

        self.blitter.prepare_unison(self.blitter.n_unison as usize);
        self.blitter.clear_buffers();

        self.pitch_last = pitch;
        self.pitch_t = pitch;
        self.update_lagvals::<true>();

        let n_tables: usize =  self.wave_wavetable.num_tables();

        //is morph the right one? 
        let shape: f32 = 
            self.pvalf(WTOscillatorParam::Morph) *
            ((n_tables as f32) - 1.0) * 
            0.999990;

        let (intpart, fracpart) = split_float(shape);

        self.tableipol = fracpart;

        self.tableid = limit_range( intpart as i32, 
            0, (n_tables - 2) as i32);

        self.last_tableipol = self.tableipol;
        self.last_tableid = self.tableid;
        self.hskew = 0.0;
        self.last_hskew = 0.0;

        if wt_flag![self,IsSample] {
            self.tableipol = 0.0;
            self.tableid -= 1;
        }

        for i in (0_usize..self.blitter.n_unison as usize).step_by(1) {

            self.blitter.oscstate[i] = 0.0;

            if !is_display {

                self.blitter.oscstate[i] = rand01();
            }

            self.blitter.state[i] = 0; 
            self.last_level[i] = 0.0;
            self.mipmap[i] = 0;
            self.mipmap_ofs[i] = 0;
            self.blitter.driftlfo2[i] = 0.0;
            self.blitter.driftlfo[i] = 0.0;
        }
    }
}
