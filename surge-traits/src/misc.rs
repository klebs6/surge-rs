pub trait HandleStreamingMismatches {
    fn handle_streaming_mismatches(
        &mut self,
        _streaming_revision: i32,
        _current_synth_streaming_revision: i32,
    ) { /*no-op here*/
    }
}

pub trait AssignFM {
    fn assign_fm(&mut self, _master_osc: *mut f32) {}
    /* { self.master_osc = master_osc; } */
}

pub trait SceneController {
    fn release_scene(&mut self, s: i32);
}

pub trait ProgramChange {
    fn program_change(&mut self, channel: char, value: i32);
}

pub trait Polyphony {
    fn enforce_polyphony_limit(&mut self, limit: i32, margin: i32);
}

pub trait FXLoader {
    fn load_fx(&mut self, initp: bool, force_reload_all: bool) -> bool ;
    fn load_fx_needed(&mut self) -> bool ;
}

pub trait InternalExternalAPIRemapper {
    fn remap_external_api_to_internal_id(&mut self, x: u32) -> i32 ;
    fn remap_internal_to_external_api_id(&mut self, x: u32) -> i32 ;
}

pub trait WaveTableController {
    fn refresh_wtlist(&mut self);
    fn get_adjacent_wavetable(&mut self, id: i32, next_prev: bool) -> i32;
    fn sort_wavetables_per_category_in_category_order(&mut self);
}
