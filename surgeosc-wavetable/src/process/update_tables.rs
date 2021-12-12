ix!();

use crate::{
    WTOscillator,
};

impl WTOscillator {

    pub fn update_tables(&mut self) { 

        let n_tables =  self.wave_wavetable.num_tables();

        // TableID-range may have changed in the meantime, check it!
        if (n_tables == 1) || (self.tableid >= (n_tables as i32)) {
            self.tableipol = 0.0;
            self.tableid = 0;
            self.last_tableid = 0;
            self.last_tableipol = 0.0;

        } else if wt_flag![self,IsSample] {

            self.tableipol = 0.0;
            self.last_tableipol = 0.0;

        } else {

            self.last_tableipol = self.tableipol;
            self.last_tableid = self.tableid;

            let mut shape: f32 = self.l_shape.v;
            shape *= ((n_tables as f32) - 1.0) * 0.999990;

            let (intpart, fracpart) = split_float(shape);
            self.tableipol = fracpart;

            self.tableid = limit_range(intpart as i32, 
                0, (n_tables - 2) as i32);

            let tableid = self.tableid;
            let last    = self.last_tableid;

            match tableid {
                _ if tableid > last => {

                    if (self.last_tableipol - 1.0).abs() < f32::EPSILON {
                        self.tableid = last;
                        self.tableipol = 1.0;
                    } else {
                        self.last_tableipol = 0.0;
                    }

                },
                _ if tableid < last => {

                    if (self.last_tableipol - 0.0).abs() < f32::EPSILON {
                        self.tableid = last;
                        self.tableipol = 0.0;
                    } else {
                        self.last_tableipol = 1.0;
                    }

                },
                _ => {},
            }
        }
    }
}
