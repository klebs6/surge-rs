ix!();

use crate::{
    N_VOCODER_VEC,
    N_VOCODER_BANDS,
    Vocoder,
    VocoderParam,
};

impl Vocoder<'sr> {

    #[inline] pub fn new_svf_vec() -> A1d::<VectorizedSvfFilter> {
        A1d::<VectorizedSvfFilter>::from_elem(N_VOCODER_VEC, VectorizedSvfFilter::default())
    }

    #[inline] pub fn new_sse_vec() -> A1d::<__m128> {
        unsafe {
            A1d::<__m128>::from_elem(N_VOCODER_VEC, z128![])
        }
    }

    pub fn new<const N: usize>( 
        tables:   &'sr TablesHandle<'sr>,
        srunit:   &'sr SampleRateHandle<'sr>,
        synth_in: &'sr SynthInputHandle<'sr>) -> Self {

        Self {
            carrier_l:     Align16(Self::new_svf_vec()),
            carrier_r:     Align16(Self::new_svf_vec()),
            modulator:     Align16(Self::new_svf_vec()),
            env_f:         Align16(Self::new_sse_vec()),
            gain:          Align16(LipolPs::new_with_blocksize(N)),
            ringout:       Ringout::blocks(500),
            params:        VocoderParam::new_runtime(), 
            bi:            0, 
            active_bands:  (N_VOCODER_BANDS as i32),
            synth_in:      synth_in.clone(),
            tables:        tables.clone(),
            srunit:        srunit.clone(),
        }
    }
}
