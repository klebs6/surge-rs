crate::ix!();

#[derive(Debug)]
pub struct ParameterMeta {
    fmin:     f32,
    fmax:     f32,
    fdefault: f32,
    flags:    ControlStyle,
    clump:    u32,
    hide:     bool,
    expert:   bool,
    meta:     bool,
}

impl Default for ParameterMeta {

    fn default() -> Self {
        todo!();
    }
}

impl ParameterMeta {

    pub fn get_fmin(&self) -> f32 {
        self.fmin
    }

    pub fn get_fmax(&self) -> f32 {
        self.fmax
    }

    pub fn get_fdefault(&self) -> f32 {
        self.fdefault
    }

    pub fn get_flags(&self) -> ControlStyle {
        self.flags
    }

    pub fn get_clump(&self) -> u32 {
        self.clump
    }

    pub fn get_hide(&self) -> bool {
        self.hide
    }

    pub fn get_expert(&self) -> bool {
        self.expert
    }

    pub fn get_meta(&self) -> bool {
        self.meta
    }
}
