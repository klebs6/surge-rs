crate::ix!();

pub trait MorphPatch {
    fn morph(&mut self);
}

pub trait UpdatePatchControls {
    // init_osc is the pointer to the 
    // data structure of a particular osc to init
    fn update_controls(&mut self, 
        init: bool, 
        loading_from_patch: bool, 
        init_osc: Option<&dyn Oscillator> );
}

pub trait PatchLoad {
    fn load_patch(&mut self, 
        data: c_void, 
        size: i32, 
        preset: bool) ;
}

pub trait PatchStore {
    fn store_patch(&mut self, 
        data: c_void) -> u32;
}

pub trait PatchLoadXML {
    fn load_xml(&mut self, 
        data: c_void, 
        size: i32, 
        preset: bool);
}

pub trait PatchStoreXML {
    fn store_xml(&mut self, data: c_void) -> u32;
}

pub trait PatchStoreRIFF {
   //unsigned int save_RIFF(void** data);
    fn store_riff(&mut self) {

    }
}
