ix!();

use crate::{
    SineWaveOscillator,
};

impl SineWaveOscillator {

    pub fn value_from_sin_and_cos(svalue: f32, cvalue: f32, shape: i32) -> f32 
    {
        let mut pvalue: f32 = svalue;

        let quadrant: i32 = match ((svalue > 0.0), (cvalue > 0.0)) {
            (true,  true)  => 1,
            (true,  false) => 2,
            (false, false) => 3,
            (false, true)  => 4,
        };

        match shape {
            1 => {
                if quadrant == 3 || quadrant == 4 {
                    pvalue = 0.0;
                }
                pvalue = 2.0 * pvalue - 1.0;
            },
            2 => {
                if quadrant == 1 || quadrant == 3 {
                    pvalue = 0.0;
                }
            },
            3 => {
                if quadrant == 2 || quadrant == 4 {
                    pvalue = 0.0;
                }
            },
            4 => {
                match quadrant {
                    1 => pvalue = 1.0 - cvalue,
                    2 => pvalue = 1.0 + cvalue,
                    3 => pvalue = -1.0 - cvalue,
                    4 => pvalue = -1.0 + cvalue,
                    _ => unreachable!(),
                }
            },
            5 => {
                match quadrant {
                    1 => pvalue = 1.0 - cvalue,
                    2 => pvalue = 1.0 + cvalue,
                    _ => pvalue = 0.0,
                }
                pvalue = 2.0 * pvalue - 1.0;
            },
            6 => {
                if quadrant <= 2 {
                    pvalue = 2.0 * svalue * cvalue; // remember sin 2x = 2 * sinx * cosx
                } else {
                    pvalue = 0.0;
                }
            },
            7 => {
                pvalue = 2.0 * svalue * cvalue;
                if quadrant == 2 || quadrant == 3 {
                    pvalue = -pvalue;
                }
            },
            8 => {
                pvalue = 2.0 * svalue * cvalue;
                match quadrant {
                    2 | 4 => pvalue = 0.0,
                    3 => pvalue = -pvalue,
                    _ => {},
                }
            },
            _ => {}
        }
        pvalue
    }
}
