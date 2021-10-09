ix!();

use crate::{
    SineWaveOscillator,
    SineWaveOscillatorParam,
};

impl HandleStreamingMismatches for SineWaveOscillator<'sr> {

    fn handle_streaming_mismatches(&mut self, 
        streaming_revision: i32, 
        _current_synth_streaming_revision: i32) 
    {
        if streaming_revision <= 10 {
            self.params[SineWaveOscillatorParam::Feedback].val = PData::Float(0.0);
            self.params[SineWaveOscillatorParam::FMBehavior].val = PData::Int(0);
        }

        if streaming_revision <= 9 {
            self.params[SineWaveOscillatorParam::Shape].val = PData::Int(0);
        }
    }
}

