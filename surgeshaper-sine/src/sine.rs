crate::ix!();

#[derive(Debug)]
pub struct SineShaper { 
    pub tables: TablesHandle,
}

impl SineShaper {
    pub fn new(tables: &TablesHandle) -> Self {
        Self {
            tables: tables.clone()
        }
    }
}
