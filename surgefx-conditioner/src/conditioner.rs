ix!();

use crate::{
    ConditionerParam,
    ConditionerParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Conditioner<'sr> {
    pub amp_l:             Align16<LipolPs>,
    pub amp_r:             Align16<LipolPs>,
    pub width:             Align16<LipolPs>,
    pub postamp:           Align16<LipolPs>,
    pub ringout:           Ringout,
    pub params:            ConditionerParamArrayRT,
    pub band1:             BiquadFilter<'sr>,
    pub band2:             BiquadFilter<'sr>,
    pub ef:                f32,
    pub a_rate:            LiPol<f32>,
    pub r_rate:            LiPol<f32>,
    pub lamax:             A1d::<f32>,
    pub delayed:           A2d::<f32>,
    pub bufpos:            i32,
    pub filtered_lamax:    f32,
    pub filtered_lamax2:   f32,
    pub gain:              f32,
    pub tables:            TablesHandle<'sr>,
    pub srunit:            SampleRateHandle<'sr>,

    /// stereo pairs, just use every other when mono
    pub vu:                A1d::<f32>, 
}

effect!          [Conditioner<'sr>, ConditionerParam];
name!            [Conditioner<'sr>,    "conditioner"];
no_op!           [Conditioner<'sr>,          Suspend];
default_default! [Conditioner<'sr>                  ];
