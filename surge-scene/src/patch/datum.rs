ix!();

/// seems to be missing from VST2.3, so it's copied from the VST list instead
///--------------------------------------------------------------------
/// For Preset (Program) (.fxp) with chunk (magic = 'FPCh')
///--------------------------------------------------------------------
#[derive(Debug,Serialize,Deserialize)]
pub struct FxChunkSetCustom {

    // 'CcnK'
    pub chunk_magic:    u32, 

    // of this chunk, excl magic + byte_size
    pub byte_size:      u32, 

    // 'FPCh'
    pub fx_magic:       u32, 

    pub version:        u32,

    // fx unique id
    pub fx_id:          u32, 
    pub fx_version:     u32,
    pub num_programs:   u32,
    pub program_name:   [char; 28],
    pub chunk_size:     PatchDataSize,
}

default_default![FxChunkSetCustom];

#[derive(Debug)]
pub struct PatchCategory {
    pub name:           String,
    pub order:          i32,
    pub children:       Vec<PatchCategory>,
    pub is_root:        bool,
    pub internalid:     i32,
    pub number_of_patches_in_category:              i32,
    pub number_of_patches_in_category_and_children: i32,
}

#[derive(Debug)]
pub struct PatchMetadata {
    pub name:                             String,
    pub path:                             Option<Box<std::path::Path>>,
    pub order:                            i32,
    pub fav:                              bool,
    pub category:                         String,
    pub author:                           String,
    pub comment:                          String,
    pub custom_controller_label:          [[char; 16]; N_CUSTOMCONTROLLERS],
    pub streaming_revision:               i32,
    pub current_synth_streaming_revision: i32,
}

impl Default for PatchMetadata {
    fn default() -> Self {
        Self {
            name:                             "init".into(),
            path:                             None,
            order:                            -1,
            fav:                              false,
            category:                         "".into(),
            author:                           "".into(),
            comment:                          "".into(),
            custom_controller_label:          [['x'; 16]; N_CUSTOMCONTROLLERS],
            streaming_revision:               -1,
            current_synth_streaming_revision: -1,
        }
    }
}

#[derive(Debug)]
pub struct Patch {
    pub name:     String,
    pub path:     Box<std::path::Path>,
    pub category: i32,
    pub order:    i32,
    pub fav:      bool,
}

compare_by![Patch,name];

pub fn is_valid_patchname(patch_name: String) -> bool {
    //maybe this needs to be different
    let re = Regex::new(r"([[:alnum:]] | [[:punct:]] | [[:space:]])+").unwrap();
    re.find_iter(&patch_name).count() != 0
}
