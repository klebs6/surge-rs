use crate::imports::*;

use crate::{
    BLOCK_SIZE,
    ControllerModulationSource,
    ModSource,
    ModulationRouting,
    N_INPUTS,
    N_OUTPUTS,
    PluginLayer,
};
//----------------------------------------------modunit

pub trait ModUnitSet {
    fn check_scene_modsource_enabled(&self, scene: usize, modsrc: ModSource) -> bool;
    fn toggle_scene_modsource(&mut self, scene: usize, modsrc: ModSource, state: bool);
    fn set_modulation(&mut self, ptag: i64, modsource: ModSource, val: f32) -> bool;
}

pub trait ModUnitQuery {
    fn is_mod_dest_used(&mut self, ptag: i64) -> bool;
    fn is_active_modulation(&mut self, ptag: i64, modsource: ModSource) -> bool;
    fn is_bipolar_modulation(&mut self, tms: ModSource) -> bool;
    fn is_valid_modulation(&mut self, ptag: usize, modsource: ModSource) -> bool;
}

pub trait ModUnitPrepare {
    fn prepare_modsource_do_process(&mut self, scenemask: i32);
}

pub trait ModUnitGet<'sr> {
    fn get_modulation(&mut self, ptag: usize, modsource: ModSource) -> f32;
    fn get_mod_depth(&mut self, ptag: i64, modsource: ModSource) -> f32;
    fn get_mod_routing(&mut self, index: usize, modsource: ModSource) -> Option<&mut ModulationRouting>;
    fn get_control_interpolator(&mut self, id: i32) -> Option<&mut ControllerModulationSource<'sr>>;
    fn get_free_control_interpolator_index(&mut self) -> i32;
    fn get_control_interpolator_index(&mut self, id: i32) -> i32;
}

pub trait ModUnitClear {
    fn clear_osc_modulation(&mut self, scene: i32, entry: i32);
    fn clear_modulation(&mut self, ptag: i64, modsource: ModSource);
}

pub trait ModUnit<'sr> {
    fn release_control_interpolator(&mut self, id: i32);
    fn add_control_interpolator(&mut self, id: i32, already_existed: &bool) 
        -> Option<&mut ControllerModulationSource<'sr>>;
}

//----------------------------------------------synth
pub trait PluginLayerIF {
    fn parameter_update(&mut self, _id: i32) {}
    fn set_parameter_automated(&mut self, _id: i32, _value: f32) {}
    fn update_display(&mut self){}
    fn patch_changed(&mut self){}
}

pub trait PluginLayerAccess {
    fn get_parent(&mut self) 
        -> *mut PluginLayer ;
}

pub trait PitchBend { 
    fn pitchbend(&mut self, 
        channel: char, 
        value:   i32);
}

pub trait PolyAfterTouch {
    fn poly_aftertouch(&mut self, 
        channel: char, 
        key:     i32, 
        value:   i32);
}

pub trait ChannelAfterTouch {
    fn channel_aftertouch(&mut self, 
        channel: char, 
        value:   i32);
}

pub trait MidiPolyphonicExpression {
    fn get_mpe_main_channel(&mut self, 
        voice_channel: i32, 
        key:          i32) -> i32 ;
}

pub trait ChannelController {

    fn channel_controller(&mut self, 
        channel: char, 
        cc:      i32, 
        value:   i32);

    fn calculate_channel_mask(&mut self, 
        channel: i32, 
        key:     i32) -> i32 ;
}

pub trait ModulationController {

    fn is_valid_modulation(&mut self, 
        ptag:      i64, 
        modsource: ModSource) -> bool ;

    fn is_active_modulation(&mut self, 
        ptag:      i64, 
        modsource: ModSource) -> bool;

    fn is_bipolar_modulation(&mut self, 
        modsources: ModSource) -> bool ;

    fn is_modsource_used(&mut self, 
        modsource: ModSource) -> bool ;

    fn is_mod_dest_used(&mut self, 
        moddest: i64) -> bool ;

    fn get_mod_routing(&mut self, 
        ptag: i64, 
        modsource: ModSource) -> Option<&mut ModulationRouting> ;

    fn set_modulation(&mut self, 
        ptag:      i64, 
        modsource: ModSource, 
        value:     f32) -> bool ;

    fn get_modulation(&mut self, 
        ptag:      i64, 
        modsource: ModSource) -> f32 ;

    fn get_mod_depth(&mut self, 
        ptag:      i64, 
        modsource: ModSource) -> f32 ;

    fn clear_modulation(&mut self, 
        ptag:      i64, 
        modsource: ModSource);

    /// clear the modulation routings on 
    /// the algorithm-specific sliders
    fn clear_osc_modulation(&mut self, 
        scene: i32, 
        entry: i32); 

    fn prepare_modsource_do_process(&mut self, 
        scenemask: i32);
}

pub trait HoldPedalController {
    fn purge_holdbuffer(&mut self, 
        scene: i32);
}

pub trait StoreDAWState {
    fn populate_daw_extra_state(&mut self);
}

pub trait LoadDAWState {
    fn load_from_daw_extra_state(&mut self);
}

pub trait GetInputs {
    fn get_num_inputs(&mut self) 
        -> i32 { N_INPUTS.try_into().unwrap() }
}

pub trait GetOutputs {
    fn get_num_outputs(&mut self) 
        -> i32 { N_OUTPUTS.try_into().unwrap() }
}

pub trait GetBlockSize {
    fn get_block_size(&mut self) 
        -> i32 { BLOCK_SIZE.try_into().unwrap() }
}
//----------------------------------------------modunit

