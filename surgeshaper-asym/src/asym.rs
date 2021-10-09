ix!();

#[derive(Debug)]
pub struct AsymShaper<'sr> {
    pub tables: TablesHandle<'sr>,
}

impl AsymShaper<'sr> {
    pub fn new(tables: &TablesHandle<'sr>) -> Self {
        Self {
            tables: tables.clone()
        }
    }
}

