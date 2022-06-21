crate::ix!();

#[derive(Debug)]
pub struct AsymShaper {
    pub tables: TablesHandle,
}

impl AsymShaper {
    pub fn new(tables: &TablesHandle) -> Self {
        Self {
            tables: tables.clone()
        }
    }
}

