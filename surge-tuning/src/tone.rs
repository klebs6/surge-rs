crate::ix!();

enhanced_enum![
    ToneType {
        Cents,
        Ratio,
    }
];

impl Default for Tone {
    fn default() -> Self {
        Self {
            ty:       ToneType::Ratio,
            cents:    0.0,
            ratio_d:  1.0,
            ratio_n:  1.0,
            repr:     "1/1".to_string(),
            val:      1.0,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Tone {
    pub ty:          ToneType,
    pub cents:       f32,
    pub ratio_d:     f32,
    pub ratio_n:     f32,
    pub repr:        String,
    pub val:         f32,
}

impl fmt::Display for Tone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let s_ty = match self.ty {
            ToneType::Cents => "cents",
            ToneType::Ratio => "ratio",
        };

        let s_val = match self.ty {
            ToneType::Cents => format!("{}",self.cents),
            ToneType::Ratio => format!("{} / {}", self.ratio_n, self.ratio_d),
        };

        let s_tup = format!("(f={} c={})", self.val, self.cents);

        write!(f, "{} {} {}", s_ty, s_val, s_tup)
    }
}
