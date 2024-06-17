crate::ix!();

impl HandleStreamingMismatches for SineWaveOscillator {

    fn handle_streaming_mismatches(&mut self, 
        streaming_revision: i32, 
        _current_synth_streaming_revision: i32) 
    {
        if streaming_revision <= 10 {
            self.params[SineWaveOscillatorParam::Feedback].set_value(PData::Float(0.0));
            self.params[SineWaveOscillatorParam::FMBehavior].set_value(PData::Int(0));
        }

        if streaming_revision <= 9 {
            self.params[SineWaveOscillatorParam::Shape].set_value(PData::Int(0));
        }
    }
}

