ix!();

#[derive(Debug)]
pub struct SineShaper<'sr> { 
    pub tables: TablesHandle<'sr>,
}

impl SineShaper<'sr> {
    pub fn new(tables: &TablesHandle<'sr>) -> Self {
        Self {
            tables: tables.clone()
        }
    }
}
