crate::ix!();

impl HandleStreamingMismatches for Distortion {

    fn handle_streaming_mismatches(&mut self, 
        streaming_revision:                i32, 
        _current_synth_streaming_revision: i32) 
    {
        if streaming_revision <= 11 
        {
            self.params[DistortionParam::Waveshaper].val        = PData::Int(0);
            self.params[DistortionParam::PreGain].extend_range  = false;
            self.params[DistortionParam::PostGain].extend_range = false;
        }
    }
}

