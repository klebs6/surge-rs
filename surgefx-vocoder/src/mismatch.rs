ix!();

use crate::{
    Vocoder,
    VocoderParam,
    VOCODER_FREQ_VSM201,
    N_VOCODER_BANDS,
};

impl HandleStreamingMismatches for Vocoder<'sr> {

    fn handle_streaming_mismatches(&mut self, 
        streaming_revision: i32, 
        _current_synth_streaming_revision: i32) 
    {
        if streaming_revision <= 10 {
            self.params[VocoderParam::NumBands].val = PData::Int(N_VOCODER_BANDS as i32);

            let f_lo: f32 = 12.0 * (VOCODER_FREQ_VSM201[0]/(CONCERT_A_HZ as f32)).log2();
            let f_hi: f32 = 12.0 * (VOCODER_FREQ_VSM201[N_VOCODER_BANDS-1]/(CONCERT_A_HZ as f32)).log2();

            self.params[VocoderParam::FreqLo].val = PData::Float(f_lo);
            self.params[VocoderParam::FreqHi].val = PData::Float(f_hi);

            self.params[VocoderParam::ModExpand].val = PData::Int(0);
            self.params[VocoderParam::ModCenter].val = PData::Int(0);
        }
    }
}

