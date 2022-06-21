crate::ix!();

pub fn maxi(a: i32, b: i32) -> i32 {
    std::cmp::max(a,b)
}

pub fn maxd(a: f64, b: f64) -> f64 {
    std::cmp::max(FloatOrd(a),FloatOrd(b)).0
}

pub fn maxf(a: f32, b: f32) -> f32 {
    std::cmp::max(FloatOrd(a),FloatOrd(b)).0
}

pub fn maxu(a: u32, b: u32) -> u32 {
    std::cmp::max(a,b)
}

//--------------------------

pub fn minf(a: f32, b: f32) -> f32 {
    std::cmp::min(FloatOrd(a),FloatOrd(b)).0
}

pub fn mind(a: f64, b: f64) -> f64 {
    std::cmp::min(FloatOrd(a),FloatOrd(b)).0
}

pub fn mini(a: i32, b: i32) -> i32 {
    std::cmp::min(a,b)
}

pub fn minu(a: u32, b: u32) -> u32 {
    std::cmp::min(a,b)
}
