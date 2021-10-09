ix!();

use crate::{
    Allpass,
    OnePoleFilter,
    AllpassDelay,
    AllpassPreDelay,
    AllpassReverbParam,
};

impl crate::AllpassVerb<'sr> {

    pub fn new( srunit: &'sr SampleRateHandle<'sr> ) -> Self {
        Self {
            mix:                    Align16(LipolPs::default()),
            width:                  Align16(LipolPs::default()),
            ringout:                Ringout::blocks(RINGOUT_DEFAULT),
            params:                 AllpassReverbParam::new_runtime(),
            input_allpass:          Self::new_input_allpass_grid(),
            allpass:                Self::new_allpass_grid(),
            hf_damper:              Self::new_hf_damper(),
            lf_damper:              Self::new_lf_damper(),
            delay:                  Self::new_delay(),
            predelay:               AllpassPreDelay::default(),
            tap_time_l:             Self::new_tap::<i32>(),
            tap_time_r:             Self::new_tap::<i32>(),
            tap_gain_l:             Self::new_tap::<f32>(),
            tap_gain_r:             Self::new_tap::<f32>(),
            state:                  0.0,
            decay_multiply:         LiPol::<f32>::default(),
            diffusion:              LiPol::<f32>::default(),
            buildup:                LiPol::<f32>::default(),
            hf_damp_coefficient:    LiPol::<f32>::default(),
            lf_damp_coefficient:    LiPol::<f32>::default(),
            modulation:             LiPol::<f32>::default(),
            lfo:                    QuadrOsc::default(),
            last_decay_time:        0.0,
            srunit:                 srunit.clone(),
        }
    }

    #[inline] fn new_allpass_grid() -> A2d::<Allpass> {
        A2d::<Allpass>::from_elem((ALLPASS_REVERB_NUM_BLOCKS,ALLPASS_REVERB_NUM_ALLPASSES_PER_BLOCK), Allpass::default())
    }

    #[inline] fn new_input_allpass_grid() -> A1d::<Allpass> {
        A1d::<Allpass>::from_elem(ALLPASS_REVERB_NUM_INPUT_ALLPASSES, Allpass::default())
    }

    #[inline] fn new_hf_damper() -> A1d::<OnePoleFilter> {
        A1d::<OnePoleFilter>::from_elem(ALLPASS_REVERB_NUM_BLOCKS, OnePoleFilter::default())
    }

    #[inline] fn new_lf_damper() -> A1d::<OnePoleFilter> {
        A1d::<OnePoleFilter>::from_elem(ALLPASS_REVERB_NUM_BLOCKS, OnePoleFilter::default())
    }

    #[inline] fn new_delay() -> A1d::<AllpassDelay> {
        A1d::<AllpassDelay>::from_elem(ALLPASS_REVERB_NUM_BLOCKS, AllpassDelay::default())
    }

    #[inline] fn new_tap<T: Zero + Clone>() -> A1d::<T> {
        A1d::<T>::from_elem(ALLPASS_REVERB_NUM_BLOCKS,  T::zero())
    }

}
