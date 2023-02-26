crate::ix!();

#[derive(Debug,Clone)]
pub struct QuadFilterUnitState {

    /// coefficients
    pub coeff:   [__m128; N_COEFFMAKER_COEFFS],
    pub dcoeff:  [__m128; N_COEFFMAKER_COEFFS],

    /// registers
    pub reg:     [__m128; N_FILTER_REGISTERS],

    //TODO: fix this -- should not be raw ptr
    /// delay buffers
    pub delay_buffer: [*mut f32; 4],

    /// 0xffffffff if voice is active, 0 if not (usable as mask)
    pub active: [u32; 4],

    /// comb write position
    pub comb_write_position: [i32; 4], 

    pub tables: TablesHandle,
}

impl QuadFilterUnitState {

    #[inline] pub fn reg(&self, base: usize, offset: usize) -> __m128 {
        self.reg[base + offset]
    }

    #[inline] pub fn reg_mut<'a>(&'a mut self, base: usize, reg_offset: usize) -> &'a mut __m128 {
        &mut self.reg[base + reg_offset]
    }

    pub fn new(
        tables: &TablesHandle
    ) -> Self {

        let z = unsafe { z128![] };

        Self {
            coeff:               [z; N_COEFFMAKER_COEFFS],
            dcoeff:              [z; N_COEFFMAKER_COEFFS],
            reg:                 [z; N_FILTER_REGISTERS],
            delay_buffer:        [std::ptr::null_mut(); 4],
            active:              [0; 4],
            comb_write_position: [0; 4], 
            tables:              tables.clone(),
        }
    }
}
