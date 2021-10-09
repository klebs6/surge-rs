ix!();

pub trait LoadPreset {
    type PresetType;
    fn load_preset(&mut self, 
        preset: Self::PresetType);
}

pub trait Save {
    fn save(&mut self);
}

pub trait SaveInto {
    fn save_into(&mut self, bytes: &mut Vec<u8>) -> PatchDataSize;
}

pub trait StoreSnapshots {
    fn store_snapshots();
}

