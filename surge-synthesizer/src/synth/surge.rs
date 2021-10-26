ix!();

use crate::{
    PluginLayer,
    SynthControl,
    FXUnit,
};

enhanced_enum![VoiceUser {
    NoUser,
    SceneA,
    SceneB,
}];

lazy_static!{
    pub static ref CS_MOD_ROUTING: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
}

#[derive(Debug)]
#[repr(align(16))]
pub struct SurgeSynthesizer<,'plugin_layer,'synth_out> 
{
    pub amp:                      Align16<LipolPs>,
    pub amp_mute:                 Align16<LipolPs>,

    pub synth_out:                SynthOutputHandle<'synth_out, BLOCK_SIZE>,
    pub synth_in:                 SynthInputHandle,

    pub tuner:                    TunerHandle,
    pub tables:                   TablesHandle,
    pub srunit:                   SampleRateHandle,
    pub timeunit:                 TimeUnitHandle,
    pub hold_pedal_unit:          HoldPedalUnitHandle,
    pub midi_unit:                MIDIUnitHandle,
    pub mpe_unit:                 MPEUnitHandle,

    pub plugin_layer:             PluginLayer<'plugin_layer>,

    pub controllers:              [i32; N_CUSTOMCONTROLLERS],

    pub halfband_in:               HalfRateFilterSSE,

    pub cc0:                      i32,
    pub pch:                      i32,

    pub controller:               SynthControl,
    pub fx_unit:                  FXUnit,

    //patch unit_____________________________________
    pub patch_loaded:             bool,
    pub patchid:                  Option<i32>,
    pub current_category_id:      Option<i32>, //0
    pub patchid_queue:            Option<i32>,
    pub active_patch:             Box<SurgePatch>,
    pub patches:                  Vec<Patch>,
    pub patch_categories:         Vec<PatchCategory>,
    pub active_patch_category:    Vec<PatchCategory>,
    pub first_3p_category:        i32,
    pub first_user_category:      i32,
    pub patch_ordering:           Vec<i32>,
    pub patch_category_ordering:  Vec<i32>,
    pub audio_processing_active:  bool,
}

