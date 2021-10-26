ix!();

use crate::*;

impl RemapToStandardKeyboard for SurgeTuner {

    fn remap_to_standard_keyboard(&mut self) -> bool 
    {
        self.remap_to_keyboard(&KeyboardMapping::default())
    }
}

impl RemapKeyboard for SurgeTuner {

    fn remap_to_keyboard(&mut self, mapping: &KeyboardMapping) -> bool 
    {
        self.current_mapping = Align16(mapping.clone());

        if self.current_mapping.is_standard_mapping {

            self.current_tuning.pitch = 32.0;
            self.current_tuning.pitch_inv = 1.0 / 32.0;

        } else {

            const SCALE: f64 = 8.175798915;

            self.current_tuning.pitch     = mapping.tuning_frequency / (SCALE as f32);
            self.current_tuning.pitch_inv = 1.0 / self.current_tuning.pitch;
        }

        // The mapping will change all the cached pitches so we need to redo
        if self.current_mapping.is_standard_mapping 
            && self.current_tuning.is_standard_tuning 
        {
            self.retune_to_standard_tuning();

        } 
        else if !self.current_mapping.is_standard_mapping 
            && !self.current_scale.is_valid() 
        {
            // We need to set the current scale to a default scale so we can remap the keyboard.
            // This is an odd edge case but we do need to support it.
            self.retune_to_scale(&Scale::even_temperament_12_note_scale());

        } else {
            let scale = self.current_scale.clone();
            self.retune_to_scale(&scale);

        }

        true
    }
}
