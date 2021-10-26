ix!();

#[derive(Debug)]
pub struct FilterBlockState {
    pub fbq:    *mut QuadFilterChainState,
    pub fbqi:   i32,
}

impl Default for FilterBlockState {
    fn default() -> Self {
        todo!();
    }
}

#[derive(Debug)]
pub struct FilterBlockData {
    pub id_cfa:        i32,
    pub id_cfb:        i32, 
    pub id_kta:        i32, 
    pub id_ktb:        i32, 
    pub id_emoda:      i32, 
    pub id_emodb:      i32, 
    pub id_resoa:      i32, 
    pub id_resob:      i32, 
    pub id_drive:      i32, 
    pub id_vca:        i32,
    pub id_vcavel:     i32, 
    pub id_fbalance:   i32, 
    pub id_feedback:   i32,
}

impl Default for FilterBlockData {
    fn default() -> Self {
        todo!();
    }
}
